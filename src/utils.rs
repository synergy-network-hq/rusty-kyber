//! Utility functions: sampling, hashing, PRFs/XOFs, (de)compression, and
//! message <-> polynomial maps used by Kyber.
//!
//! Notes:
//! - Hashing uses `sha3` crate (SHA3-256, SHA3-512, SHAKE128, SHAKE256).
//! - Rejection sampling and CBD (η = 2/3) are implemented per spec.
//! - Generic `d`-bit (de)compression underlies DU/DV packing.

use crate::params::{N, Q, DU, DV};
use crate::poly::Poly;
use sha3::{Digest, Sha3_256, Sha3_512, Shake128, Shake256};
use sha3::digest::{ExtendableOutput, XofReader};

#[inline(always)]
fn montgomery_reduce(a: i32) -> i16 {
    // r = a * R^{-1} mod Q with R=2^16; Q_INV = -Q^{-1} mod 2^16 = 62209
    let u = (a as i64 * 62209i64) as i32 & 0xFFFF;
    let t = (a - u * Q) >> 16;
    t as i16
}

#[inline(always)]
fn barrett_reduce(a: i16) -> i16 {
    // Conservative Barrett; used during serialization to keep values in range.
    let v = ((1u32 << 26) + (Q as u32 / 2)) / (Q as u32);
    let t = ((v as u32 * (a as u32)) >> 26) as i16;
    a - t * (Q as i16)
}

/// Centered binomial distribution sampler for `eta ∈ {2,3}`.
///
/// - For `eta=2`, consumes 4 bytes per 8 coefficients.
/// - For `eta=3`, consumes 3 bytes per 4 coefficients.
#[inline]
pub fn cbd_eta(buf: &[u8], eta: i32, r: &mut Poly) {
    if eta == 2 {
        let mut j = 0usize;
        for i in 0..N / 8 {
            let t = u32::from_le_bytes([buf[4*i], buf[4*i+1], buf[4*i+2], buf[4*i+3]]);
            let d = (t & 0x5555_5555) + ((t >> 1) & 0x5555_5555);
            for k in 0..8 {
                let a = ((d >> (4*k)) & 0x3) as i16;
                let b = ((d >> (4*k + 2)) & 0x3) as i16;
                r.coeffs[j] = a - b;
                j += 1;
            }
        }
    } else {
        // eta == 3
        let mut j = 0usize;
        for i in 0..N / 4 {
            let t = u32::from_le_bytes([buf[3*i], buf[3*i+1], buf[3*i+2], 0]);
            let d = (t & 0x2492_4924) + ((t >> 1) & 0x2492_4924) + ((t >> 2) & 0x2492_4924);
            for k in 0..4 {
                let a = ((d >> (6*k)) & 0x7) as i16;
                let b = ((d >> (6*k + 3)) & 0x7) as i16;
                r.coeffs[j] = a - b;
                j += 1;
            }
        }
    }
}

/// Backward-compatible alias with `eta=2`.
#[inline]
pub fn cbd(buf: &[u8], r: &mut Poly) { cbd_eta(buf, 2, r) }

/// `H`: SHA3-256(m) -> 32 bytes.
#[inline(always)]
pub fn h(input: &[u8], out: &mut [u8; 32]) {
    let mut hasher = Sha3_256::new();
    Digest::update(&mut hasher, input);
    let digest = hasher.finalize();
    out.copy_from_slice(&digest);
}

/// `G`: SHA3-512(m) -> 64 bytes.
#[inline(always)]
pub fn g(input: &[u8], out: &mut [u8; 64]) {
    let mut hasher = Sha3_512::new();
    Digest::update(&mut hasher, input);
    let digest = hasher.finalize();
    out.copy_from_slice(&digest);
}

/// `KDF`: SHAKE256(k || hash) -> 32 bytes.
#[inline(always)]
pub fn kdf(input: &[u8], out: &mut [u8; 32]) {
    let mut x = Shake256::default();
    sha3::digest::Update::update(&mut x, input);
    let mut rdr = x.finalize_xof();
    rdr.read(out);
}

/// `PRF`: SHAKE256(seed || nonce) -> `out.len()` bytes.
#[inline(always)]
pub fn prf(seed: &[u8], nonce: u8, out: &mut [u8]) {
    let mut x = Shake256::default();
    sha3::digest::Update::update(&mut x, seed);
    sha3::digest::Update::update(&mut x, &[nonce]);
    let mut rdr = x.finalize_xof();
    rdr.read(out);
}

/// Matrix expansion XOF: SHAKE128(rho || i || j) → stream for A[i,j].
#[inline(always)]
pub fn xof_matrix(rho: &[u8; 32], i: u8, j: u8, out: &mut [u8]) {
    let mut x = Shake128::default();
    sha3::digest::Update::update(&mut x, rho);
    sha3::digest::Update::update(&mut x, &[i, j]);
    let mut rdr = x.finalize_xof();
    rdr.read(out);
}

/// Rejection sampling into `[0, Q)` using packed 12-bit candidates.
#[inline]
pub fn rej_uniform(buf: &[u8], r: &mut Poly) {
    let mut ctr = 0usize;
    let mut pos = 0usize;
    while ctr < N {
        if pos + 3 > buf.len() { break; }
        let val0 = (buf[pos] as u16) | (((buf[pos+1] as u16) & 0x0F) << 8);
        let val1 = ((buf[pos+1] as u16) >> 4) | ((buf[pos+2] as u16) << 4);
        pos += 3;
        if val0 < Q as u16 {
            r.coeffs[ctr] = val0 as i16;
            ctr += 1;
        }
        if ctr < N && val1 < Q as u16 {
            r.coeffs[ctr] = val1 as i16;
            ctr += 1;
        }
    }
}

/// Serialize a polynomial into 12-bit packed form (384 bytes).
#[inline]
pub fn poly_to_bytes(a: &Poly, out: &mut [u8; 384]) {
    for i in 0..N/8 {
        let mut t = [0i16; 8];
        for j in 0..8 { t[j] = a.coeffs[8*i + j]; }
        for j in 0..8 {
            t[j] = barrett_reduce(t[j]);
            // Canonicalize to [0, q)
            let mut v = t[j] as i32;
            // Add q if negative (branchless)
            v += (Q & (v >> 31));
            // If v >= q, subtract q (branchless)
            let ge = (((v - Q) >> 31) ^ -1) & 1; // 1 if v >= Q else 0
            v -= ge * Q;
            t[j] = v as i16;
        }
        let p0 = t[0] as u16; let p1 = t[1] as u16; let p2 = t[2] as u16; let p3 = t[3] as u16;
        let p4 = t[4] as u16; let p5 = t[5] as u16; let p6 = t[6] as u16; let p7 = t[7] as u16;

        out[13*i+0]  = (p0 & 0xFF) as u8;
        out[13*i+1]  = ((p0 >> 8) | ((p1 & 0x1F) << 5)) as u8;
        out[13*i+2]  = ((p1 >> 3) & 0xFF) as u8;
        out[13*i+3]  = ((p1 >> 11) | ((p2 & 0x03) << 2) | ((p3 & 0x07) << 7)) as u8;
        out[13*i+4]  = ((p2 >> 6) & 0xFF) as u8;
        out[13*i+5]  = ((p2 >> 14) | ((p3 & 0xFF) << 1)) as u8;
        out[13*i+6]  = ((p3 >> 7) | ((p4 & 0x0F) << 4)) as u8;
        out[13*i+7]  = ((p4 >> 4) & 0xFF) as u8;
        out[13*i+8]  = ((p4 >> 12) | ((p5 & 0x7F) << 1)) as u8;
        out[13*i+9]  = ((p5 >> 7) | ((p6 & 0x3F) << 6)) as u8;
        out[13*i+10] = ((p6 >> 2) & 0xFF) as u8;
        out[13*i+11] = ((p6 >> 10) | ((p7 & 0x1F) << 3)) as u8;
        out[13*i+12] = ((p7 >> 5) & 0xFF) as u8;
    }
}

/// Deserialize a polynomial from 12-bit packed form (384 bytes).
#[inline]
pub fn poly_from_bytes(inp: &[u8; 384], r: &mut Poly) {
    for i in 0..N/8 {
        let b = &inp[13*i..13*i+13];
        let t0 = (b[0] as u16) | (((b[1] as u16) & 0x1F) << 8);
        let t1 = ((b[1] as u16) >> 5) | ((b[2] as u16) << 3) | (((b[3] as u16) & 0x03) << 11);
        let t2 = ((b[3] as u16) >> 2) | (((b[4] as u16) & 0x7F) << 6);
        let t3 = ((b[4] as u16) >> 7) | ((b[5] as u16) << 1) | (((b[6] as u16) & 0x0F) << 9);
        let t4 = ((b[6] as u16) >> 4) | ((b[7] as u16) << 4) | (((b[8] as u16) & 0x01) << 12);
        let t5 = ((b[8] as u16) >> 1) | (((b[9] as u16) & 0x3F) << 7);
        let t6 = ((b[9] as u16) >> 6) | ((b[10] as u16) << 2) | (((b[11] as u16) & 0x07) << 10);
        let t7 = ((b[11] as u16) >> 3) | ((b[12] as u16) << 5);

        r.coeffs[8*i+0] = (t0 as i32 % Q) as i16;
        r.coeffs[8*i+1] = (t1 as i32 % Q) as i16;
        r.coeffs[8*i+2] = (t2 as i32 % Q) as i16;
        r.coeffs[8*i+3] = (t3 as i32 % Q) as i16;
        r.coeffs[8*i+4] = (t4 as i32 % Q) as i16;
        r.coeffs[8*i+5] = (t5 as i32 % Q) as i16;
        r.coeffs[8*i+6] = (t6 as i32 % Q) as i16;
        r.coeffs[8*i+7] = (t7 as i32 % Q) as i16;
    }
}

/// Generic `d`-bit compression used by DU/DV codecs.
#[inline]
pub fn poly_compress_d(a: &Poly, d: usize, out: &mut [u8]) {
    let mut acc: u32 = 0;
    let mut acc_bits: usize = 0;
    let mask = (1u32 << d) - 1;
    let scale = ((1u32 << d) as u64);

    let mut pos = 0usize;
    for &c in a.coeffs.iter() {
        let mut v = c as i32;
        // Canonicalize to [0, q) without branches
        v += (Q & (v >> 31));
        let ge = (((v - Q) >> 31) ^ -1) & 1; // 1 if v >= Q
        v -= ge * Q;
        // Round/scale to d bits
        let t = (((v as u64) * scale + (Q as u64 / 2)) / (Q as u64)) as u32;

        acc |= (t & mask) << acc_bits;
        acc_bits += d;

        while acc_bits >= 8 {
            out[pos] = (acc & 0xFF) as u8;
            pos += 1;
            acc >>= 8;
            acc_bits -= 8;
        }
    }
    if acc_bits > 0 {
        out[pos] = (acc & 0xFF) as u8;
    }
}

/// Generic `d`-bit decompression used by DU/DV codecs.
#[inline]
pub fn poly_decompress_d(inp: &[u8], d: usize, r: &mut Poly) {
    let mut acc: u32 = 0;
    let mut acc_bits: usize = 0;
    let mut pos = 0usize;
    let mask = (1u32 << d) - 1;
    let q_u64 = Q as u64;

    for i in 0..N {
        while acc_bits < d {
            acc |= (inp[pos] as u32) << acc_bits;
            acc_bits += 8;
            pos += 1;
        }
        let t = acc & mask;
        acc >>= d;
        acc_bits -= d;
        // Scale back to [0, q)
        let val = ((t as u64) * q_u64 + ((1u64 << d) / 2)) >> d;
        r.coeffs[i] = val as i16;
    }
}

/// DU (u-vector) compressor wrapper.
#[inline(always)] pub fn poly_compress_u(a: &Poly, out: &mut [u8]) { poly_compress_d(a, DU, out) }
/// DU (u-vector) decompressor wrapper.
#[inline(always)] pub fn poly_decompress_u(inp: &[u8], r: &mut Poly) { poly_decompress_d(inp, DU, r) }
/// DV (v-vector) compressor wrapper.
#[inline(always)] pub fn poly_compress_v(a: &Poly, out: &mut [u8]) { poly_compress_d(a, DV, out) }
/// DV (v-vector) decompressor wrapper.
#[inline(always)] pub fn poly_decompress_v(inp: &[u8], r: &mut Poly) { poly_decompress_d(inp, DV, r) }

/// Map a 32-byte message to a polynomial with coefficients in `{0, (q+1)/2}`.
#[inline]
pub fn poly_from_msg(msg: &[u8; 32], r: &mut Poly) {
    let one = ((Q as i16) + 1) / 2;
    for i in 0..N {
        let bit = ((msg[i >> 3] >> (i & 7)) & 1) as i16;
        r.coeffs[i] = bit * one;
    }
}

/// Map a polynomial back to a 32-byte message (threshold at `q/2`).
#[inline]
pub fn poly_to_msg(a: &Poly, msg: &mut [u8; 32]) {
    for b in msg.iter_mut() { *b = 0; }
    for i in 0..N {
        let mut v = a.coeffs[i] as i32;
        v += (Q & (v >> 31));
        let ge = (((v << 1) - Q) >> 31) ^ -1; // 1 if 2v >= Q
        msg[i >> 3] |= ((ge as u8) & 1) << (i & 7);
    }
}

/// Convenience: rejection sample then NTT the polynomial (used during A*s).
#[inline]
pub fn sample_ntt(xof_out: &[u8], poly: &mut Poly) {
    rej_uniform(xof_out, poly);
    poly.ntt();
}
