//! Utility functions: hashing, XOFs, PRFs, sampling, and byte (de)serialization
//! for ML-KEM (Kyber), per FIPS 203.
//!
//! - Hashing uses the `sha3` crate (SHA3-256, SHA3-512, SHAKE128, SHAKE256).
//! - XOF for matrix expansion absorbs `ρ || j || i` (see FIPS 203 Appendix C.2).
//! - All arithmetic is constant-time and integer-only.

use crate::params::{ N, Q, DU, DV };
use crate::poly::Poly;

use sha3::{ Digest, Sha3_256, Sha3_512, Shake128, Shake256 };
use sha3::digest::{ ExtendableOutput, XofReader };

// -----------------------------------------------------------------------------
// Hash / XOF wrappers per FIPS 203 §4.1 (H, G, J, PRF)
// -----------------------------------------------------------------------------

/// H(s) = SHA3-256(s) -> 32 bytes
#[inline(always)]
pub fn h(input: &[u8], out: &mut [u8; 32]) {
    let mut hasher = Sha3_256::new();
    sha3::Digest::update(&mut hasher, input);
    let digest = hasher.finalize();
    out.copy_from_slice(&digest);
}

/// G(c) = SHA3-512(c) -> 64 bytes
#[inline(always)]
pub fn g(input: &[u8], out: &mut [u8; 64]) {
    let mut hasher = Sha3_512::new();
    sha3::Digest::update(&mut hasher, input);
    let digest = hasher.finalize();
    out.copy_from_slice(&digest);
}

/// J(x) = SHAKE256(x) -> 32 bytes (used as KDF)
#[inline(always)]
pub fn kdf(input: &[u8], out: &mut [u8; 32]) {
    let mut x = Shake256::default();
    sha3::digest::Update::update(&mut x, input);
    let mut rdr = x.finalize_xof();
    rdr.read(out);
}

/// PRF(seed, nonce) = SHAKE256(seed || nonce) -> out.len() bytes
#[inline(always)]
pub fn prf(seed: &[u8], nonce: u8, out: &mut [u8]) {
    let mut x = Shake256::default();
    sha3::digest::Update::update(&mut x, seed);
    sha3::digest::Update::update(&mut x, &[nonce]);
    let mut rdr = x.finalize_xof();
    rdr.read(out);
}

// -----------------------------------------------------------------------------
// Matrix expansion XOF (SampleNTT) — FIPS 203 Algorithm 7 and §5/§7 usage
// -----------------------------------------------------------------------------

/// Fill `out` by squeezing SHAKE128 absorbed with ρ || j || i for Â[i,j].
///
/// FIPS 203 (final): Â[i,j] ← SampleNTT(ρ || j || i)
#[inline(always)]
pub fn xof_matrix(rho: &[u8; 32], i: u8, j: u8, out: &mut [u8]) {
    let mut x = Shake128::default();
    sha3::digest::Update::update(&mut x, rho);
    // Final ML-KEM matches CRYSTALS-Kyber index order: absorb (i, j)
    sha3::digest::Update::update(&mut x, &[i, j]);
    let mut rdr = x.finalize_xof();
    rdr.read(out);
}

/// Back-compat alias
#[inline(always)]
pub fn xof_matrix_entry(rho: &[u8; 32], i: u8, j: u8, out: &mut [u8]) {
    xof_matrix(rho, i, j, out);
}

/// Sample a polynomial uniformly in [0,q) using a streaming SHAKE128 XOF(rho || i || j)
/// until all 256 coefficients are filled (FIPS 203 Algorithm 7 behavior).
/// Returns the polynomial in NTT domain as required by FIPS 203.
#[inline]
pub fn sample_uniform_poly_from_xof(rho: &[u8; 32], i: u8, j: u8, r: &mut Poly) {
    let mut x = Shake128::default();
    sha3::digest::Update::update(&mut x, rho);
    sha3::digest::Update::update(&mut x, &[i, j]);
    let mut rdr = x.finalize_xof();

    let mut ctr = 0usize;
    let mut buf = [0u8; 3 * 128];
    while ctr < N {
        rdr.read(&mut buf);
        let mut pos = 0usize;
        while ctr < N && pos + 3 <= buf.len() {
            let d1 = (buf[pos] as u16) | (((buf[pos + 1] as u16) & 0x0f) << 8);
            let d2 = ((buf[pos + 1] as u16) >> 4) | ((buf[pos + 2] as u16) << 4);
            pos += 3;
            if d1 < (Q as u16) {
                r.coeffs[ctr] = d1 as i16;
                ctr += 1;
            }
            if ctr < N && d2 < (Q as u16) {
                r.coeffs[ctr] = d2 as i16;
                ctr += 1;
            }
        }
    }
    // Convert to NTT domain as required by FIPS 203
    r.ntt();
}

// -----------------------------------------------------------------------------
// Rejection sampling (Algorithm 7) and CBD samplers (Algorithm 8)
// -----------------------------------------------------------------------------

/// Rejection sampling into `[0, q)` using packed 12-bit candidates.
#[inline]
pub fn rej_uniform(buf: &[u8], r: &mut Poly) {
    let mut ctr = 0usize;
    let mut pos = 0usize;
    while ctr < N {
        if pos + 3 > buf.len() {
            break;
        }
        let val0 = (buf[pos] as u16) | (((buf[pos + 1] as u16) & 0x0f) << 8);
        let val1 = ((buf[pos + 1] as u16) >> 4) | ((buf[pos + 2] as u16) << 4);
        pos += 3;
        if val0 < (Q as u16) {
            r.coeffs[ctr] = val0 as i16;
            ctr += 1;
        }
        if ctr < N && val1 < (Q as u16) {
            r.coeffs[ctr] = val1 as i16;
            ctr += 1;
        }
    }
}

/// Centered binomial distribution sampler for `eta ∈ {2,3}`.
#[inline]
pub fn cbd_eta(buf: &[u8], eta: i32, r: &mut Poly) {
    if eta == 2 {
        let mut j = 0usize;
        for i in 0..N / 8 {
            let t = u32::from_le_bytes([
                buf[4 * i],
                buf[4 * i + 1],
                buf[4 * i + 2],
                buf[4 * i + 3],
            ]);
            let d = (t & 0x5555_5555) + ((t >> 1) & 0x5555_5555);
            for k in 0..8 {
                let a = ((d >> (4 * k)) & 0x3) as i16;
                let b = ((d >> (4 * k + 2)) & 0x3) as i16;
                r.coeffs[j] = a - b;
                j += 1;
            }
        }
    } else {
        let mut j = 0usize;
        for i in 0..N / 4 {
            let t = u32::from_le_bytes([buf[3 * i], buf[3 * i + 1], buf[3 * i + 2], 0]);
            let d = (t & 0x2492_4924) + ((t >> 1) & 0x2492_4924) + ((t >> 2) & 0x2492_4924);
            for k in 0..4 {
                let a = ((d >> (6 * k)) & 0x7) as i16;
                let b = ((d >> (6 * k + 3)) & 0x7) as i16;
                r.coeffs[j] = a - b;
                j += 1;
            }
        }
    }
}

/// Backward-compatible alias with `eta=2`.
#[inline]
pub fn cbd(buf: &[u8], r: &mut Poly) {
    cbd_eta(buf, 2, r)
}

// -----------------------------------------------------------------------------
// Byte encode/decode & compress/decompress helpers
// -----------------------------------------------------------------------------

#[inline(always)]
fn barrett_reduce(a: i16) -> i16 {
    let v: i32 = (((1u32 << 26) + (Q as u32) / 2) / (Q as u32)) as i32;
    let t: i32 = (v * (a as i32)) >> 26;
    ((a as i32) - t * (Q as i32)) as i16
}

/// Serialize a polynomial into 12-bit packed form (384 bytes).
#[inline]
pub fn poly_to_bytes(a: &Poly, out: &mut [u8; 384]) {
    for i in 0..N / 8 {
        let mut t = [0i16; 8];
        for j in 0..8 {
            t[j] = a.coeffs[8 * i + j];
        }
        for j in 0..8 {
            t[j] = barrett_reduce(t[j]);
            let mut v = t[j] as i32;
            v += Q & (v >> 31);
            let ge = (((v - Q) >> 31) ^ -1) & 1;
            v -= ge * Q;
            t[j] = v as i16;
        }
        let p0 = (t[0] as u16) & 0x0fff;
        let p1 = (t[1] as u16) & 0x0fff;
        let p2 = (t[2] as u16) & 0x0fff;
        let p3 = (t[3] as u16) & 0x0fff;
        let p4 = (t[4] as u16) & 0x0fff;
        let p5 = (t[5] as u16) & 0x0fff;
        let p6 = (t[6] as u16) & 0x0fff;
        let p7 = (t[7] as u16) & 0x0fff;
        let base = 12 * i;
        out[base + 0] = (p0 & 0xff) as u8;
        out[base + 1] = ((p0 >> 8) as u8) | (((p1 & 0x000f) as u8) << 4);
        out[base + 2] = ((p1 >> 4) & 0xff) as u8;
        out[base + 3] = (p2 & 0xff) as u8;
        out[base + 4] = ((p2 >> 8) as u8) | (((p3 & 0x000f) as u8) << 4);
        out[base + 5] = ((p3 >> 4) & 0xff) as u8;
        out[base + 6] = (p4 & 0xff) as u8;
        out[base + 7] = ((p4 >> 8) as u8) | (((p5 & 0x000f) as u8) << 4);
        out[base + 8] = ((p5 >> 4) & 0xff) as u8;
        out[base + 9] = (p6 & 0xff) as u8;
        out[base + 10] = ((p6 >> 8) as u8) | (((p7 & 0x000f) as u8) << 4);
        out[base + 11] = ((p7 >> 4) & 0xff) as u8;
    }
}

/// Deserialize a polynomial from 12-bit packed form (384 bytes).
#[inline]
pub fn poly_from_bytes(inp: &[u8; 384], r: &mut Poly) {
    for i in 0..N / 8 {
        let b = &inp[12 * i..12 * i + 12];
        let t0 = (b[0] as u16) | (((b[1] as u16) & 0x0f) << 8);
        let t1 = ((b[1] as u16) >> 4) | ((b[2] as u16) << 4);
        let t2 = (b[3] as u16) | (((b[4] as u16) & 0x0f) << 8);
        let t3 = ((b[4] as u16) >> 4) | ((b[5] as u16) << 4);
        let t4 = (b[6] as u16) | (((b[7] as u16) & 0x0f) << 8);
        let t5 = ((b[7] as u16) >> 4) | ((b[8] as u16) << 4);
        let t6 = (b[9] as u16) | (((b[10] as u16) & 0x0f) << 8);
        let t7 = ((b[10] as u16) >> 4) | ((b[11] as u16) << 4);
        r.coeffs[8 * i + 0] = ((t0 as i32) % Q) as i16;
        r.coeffs[8 * i + 1] = ((t1 as i32) % Q) as i16;
        r.coeffs[8 * i + 2] = ((t2 as i32) % Q) as i16;
        r.coeffs[8 * i + 3] = ((t3 as i32) % Q) as i16;
        r.coeffs[8 * i + 4] = ((t4 as i32) % Q) as i16;
        r.coeffs[8 * i + 5] = ((t5 as i32) % Q) as i16;
        r.coeffs[8 * i + 6] = ((t6 as i32) % Q) as i16;
        r.coeffs[8 * i + 7] = ((t7 as i32) % Q) as i16;
    }
}

/// Generic d-bit compression used by DU/DV codecs.
#[inline]
pub fn poly_compress_d(a: &Poly, d: usize, out: &mut [u8]) {
    let mut acc: u32 = 0;
    let mut acc_bits: usize = 0;
    let mask = (1u32 << d) - 1;
    let scale = (1u32 << d) as u64;
    let mut pos = 0usize;
    for &c in a.coeffs.iter() {
        let mut v = c as i32;
        v += Q & (v >> 31);
        let ge = (((v - Q) >> 31) ^ -1) & 1;
        v -= ge * Q;
        let t = (((v as u64) * scale + (Q as u64) / 2) / (Q as u64)) as u32;
        acc |= (t & mask) << acc_bits;
        acc_bits += d;
        while acc_bits >= 8 {
            out[pos] = (acc & 0xff) as u8;
            pos += 1;
            acc >>= 8;
            acc_bits -= 8;
        }
    }
    if acc_bits > 0 {
        out[pos] = (acc & 0xff) as u8;
    }
}

/// Generic d-bit decompression used by DU/DV codecs.
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
        let val = ((t as u64) * q_u64 + (1u64 << d) / 2) >> d;
        r.coeffs[i] = val as i16;
    }
}

/// DU (u-vector) compressor wrapper.
#[inline(always)]
pub fn poly_compress_u(a: &Poly, out: &mut [u8]) {
    poly_compress_d(a, DU, out)
}
/// DU (u-vector) decompressor wrapper.
#[inline(always)]
pub fn poly_decompress_u(inp: &[u8], r: &mut Poly) {
    poly_decompress_d(inp, DU, r)
}
/// DV (v-vector) compressor wrapper.
#[inline(always)]
pub fn poly_compress_v(a: &Poly, out: &mut [u8]) {
    poly_compress_d(a, DV, out)
}
/// DV (v-vector) decompressor wrapper.
#[inline(always)]
pub fn poly_decompress_v(inp: &[u8], r: &mut Poly) {
    poly_decompress_d(inp, DV, r)
}

/// Map a 32-byte message to a polynomial with coefficients in {0, (q+1)/2}.
#[inline]
pub fn poly_from_msg(msg: &[u8; 32], r: &mut Poly) {
    let one = ((Q as i16) + 1) / 2;
    for i in 0..N {
        let bit = ((msg[i >> 3] >> (i & 7)) & 1) as i16;
        r.coeffs[i] = bit * one;
    }
}

/// Map a polynomial back to a 32-byte message (threshold at q/2).
#[inline]
pub fn poly_to_msg(a: &Poly, msg: &mut [u8; 32]) {
    for b in msg.iter_mut() {
        *b = 0;
    }
    for i in 0..N {
        // Canonicalize v into [0, q)
        let mut v = a.coeffs[i] as i32;
        v %= Q as i32;
        if v < 0 {
            v += Q as i32;
        }
        // t = round(2*v / q) mod 2
        let t = (((v << 1) + (Q as i32) / 2) / (Q as i32)) & 1;
        msg[i >> 3] |= ((t as u8) & 1) << (i & 7);
    }
}

/// Convenience: rejection sample then NTT the polynomial (used during A*s).
#[inline]
pub fn sample_ntt(xof_out: &[u8], poly: &mut Poly) {
    rej_uniform(xof_out, poly);
    poly.ntt();
}
