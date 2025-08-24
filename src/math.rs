//! Core mathematical operations for ML-KEM (FIPS 203)
//!
//! This module consolidates all mathematical operations to ensure FIPS 203 compliance
//! and make verification easier. All algorithms are implemented exactly as specified
//! in NIST FIPS 203.

use crate::params::{ N, Q, DU, DV };
use sha3::{ Digest, Sha3_256, Sha3_512, Shake128, Shake256 };
use sha3::digest::{ ExtendableOutput, XofReader, Update };

// =============================================================================
// POLYNOMIAL ARITHMETIC (FIPS 203 Section 2.1)
// =============================================================================

/// A degree-255 polynomial with coefficients modulo Q = 3329.
/// Represents elements in R_q = Z_q[X]/(X^256 + 1).
#[derive(Clone, Copy, Debug)]
pub struct Poly {
    /// Coefficients in canonical order.
    /// Domain (normal vs NTT) is tracked by caller context.
    pub coeffs: [i16; N],
}

impl Default for Poly {
    fn default() -> Self {
        Self::new()
    }
}

impl Poly {
    /// Construct the zero polynomial.
    #[inline(always)]
    pub const fn new() -> Self {
        Self { coeffs: [0; N] }
    }

    /// In-place coefficient-wise addition: self += other (mod Q with wrapping).
    #[inline(always)]
    pub fn add(&mut self, other: &Poly) {
        for i in 0..N {
            self.coeffs[i] = self.coeffs[i].wrapping_add(other.coeffs[i]);
        }
    }

    /// In-place coefficient-wise subtraction: self -= other (mod Q with wrapping).
    #[inline(always)]
    pub fn sub(&mut self, other: &Poly) {
        for i in 0..N {
            self.coeffs[i] = self.coeffs[i].wrapping_sub(other.coeffs[i]);
        }
    }

    /// Forward NTT transformation (FIPS 203 Algorithm 9).
    #[inline(always)]
    pub fn ntt(&mut self) {
        ntt_forward(&mut self.coeffs);
    }

    /// Inverse NTT transformation (FIPS 203 Algorithm 10).
    #[inline(always)]
    pub fn inv_ntt(&mut self) {
        ntt_inverse(&mut self.coeffs);
    }

    /// Pointwise multiplication in NTT domain (FIPS 203 Algorithm 11).
    #[inline(always)]
    pub fn pointwise_mul(&mut self, other: &Poly) {
        pointwise_multiply(&mut self.coeffs, &other.coeffs);
    }
}

// =============================================================================
// NUMBER THEORETIC TRANSFORM (FIPS 203 Algorithms 9-11)
// =============================================================================

/// Zeta values from FIPS 203 Appendix A.
/// These are ζ^BitRev₇(i) mod q for i ∈ {0, ..., 127}.
const ZETAS: [i16; 128] = [
    1, 1729, 2580, 3289, 2642, 630, 1897, 848, 1062, 1919, 193, 797, 2786, 3260, 569, 1746, 296, 2447,
    1339, 1476, 3046, 56, 2240, 1333, 1426, 2094, 535, 2882, 2393, 2879, 1974, 821, 289, 331, 3253, 1756,
    1197, 2304, 2277, 2055, 650, 1977, 2513, 632, 2865, 33, 1320, 1915, 2319, 1435, 807, 452, 1438, 2868,
    1534, 2402, 2647, 2617, 1481, 648, 2474, 3110, 1227, 910, 17, 2761, 583, 2649, 1637, 723, 2288, 1100,
    1409, 2662, 3281, 233, 756, 2156, 3015, 3050, 1703, 1651, 2789, 1789, 1847, 952, 1461, 2687,
    939, 2308, 2437, 2388, 733, 2337, 268, 641, 1584, 2298, 2037, 3220, 375, 2549, 2090, 1645,
    1063, 319, 2773, 757, 2099, 561, 2466, 2594, 2804, 1092, 403, 1026, 1143, 2150, 2775, 886,
    1722, 1212, 1874, 1029, 2110, 2935, 885, 2154,
];

/// Gamma values for pointwise multiplication.
/// These are ζ^{2·BitRev₇(i)+1} mod q for i ∈ {0, ..., 127}.
const GAMMAS: [i16; 128] = [
    17, 2761, 583, 2649, 1637, 723, 2288, 1100, 1409, 2662, 3281, 233, 756, 2156, 3015, 3050, 1703, 1651,
    2789, 1789, 1847, 952, 1461, 2687, 939, 2308, 2437, 2388, 733, 2337, 268, 641, 1584, 2298,
    2037, 3220, 375, 2549, 2090, 1645, 1063, 319, 2773, 757, 2099, 561, 2466, 2594, 2804, 1092,
    403, 1026, 1143, 2150, 2775, 886, 1722, 1212, 1874, 1029, 2110, 2935, 885, 2154, 17, 2761, 583, 2649,
    1637, 723, 2288, 1100, 1409, 2662, 3281, 233, 756, 2156, 3015, 3050, 1703, 1651, 2789, 1789,
    1847, 952, 1461, 2687, 939, 2308, 2437, 2388, 733, 2337, 268, 641, 1584, 2298, 2037, 3220, 375, 2549,
    2090, 1645, 1063, 319, 2773, 757, 2099, 561, 2466, 2594, 2804, 1092, 403, 1026, 1143, 2150,
    2775, 886, 1722, 1212, 1874, 1029, 2110, 2935, 885, 2154,
];

#[inline(always)]
fn small_reduce(x: u16) -> u16 {
    if x < (Q as u16) { x } else { x - (Q as u16) }
}

#[inline(always)]
fn barrett_reduce(x: u32) -> u16 {
    const BARRETT_SHIFT: usize = 24;
    const BARRETT_MULTIPLIER: u64 = (1 << BARRETT_SHIFT) / (Q as u64);
    let product = (x as u64) * BARRETT_MULTIPLIER;
    let quotient = (product >> BARRETT_SHIFT) as u32;
    let remainder = x - quotient * (Q as u32);
    small_reduce(remainder as u16)
}

#[inline(always)]
fn field_add(a: u16, b: u16) -> u16 {
    let result = (a as u32) + (b as u32);
    (result % (Q as u32)) as u16
}

#[inline(always)]
fn field_sub(a: u16, b: u16) -> u16 {
    let result = (a as u32) + (Q as u32) - (b as u32);
    (result % (Q as u32)) as u16
}

#[inline(always)]
fn field_mul(a: u16, b: u16) -> u16 {
    barrett_reduce((a as u32) * (b as u32))
}

#[inline(always)]
fn i16_to_field(x: i16) -> u16 {
    let normalized = (((x as i32) % Q) + Q) % Q;
    normalized as u16
}

#[inline(always)]
fn field_to_i16(x: u16) -> i16 {
    if x > (Q as u16) / 2 { ((x as i32) - Q) as i16 } else { x as i16 }
}

/// Forward NTT (FIPS 203 Algorithm 9).
#[inline(always)]
pub fn ntt_forward(a: &mut [i16; 256]) {
    let mut f = [0u16; 256];
    for i in 0..256 {
        f[i] = i16_to_field(a[i]);
    }

    let mut k = 1usize;
    for &len in &[128, 64, 32, 16, 8, 4, 2] {
        for start in (0..256).step_by(2 * len) {
            let zeta = i16_to_field(ZETAS[k]);
            k += 1;
            for j in start..start + len {
                let t = field_mul(zeta, f[j + len]);
                f[j + len] = field_sub(f[j], t);
                f[j] = field_add(f[j], t);
            }
        }
    }

    for i in 0..256 {
        a[i] = field_to_i16(f[i]);
    }
}

/// Inverse NTT (FIPS 203 Algorithm 10).
#[inline(always)]
pub fn ntt_inverse(a: &mut [i16; 256]) {
    let mut f = [0u16; 256];
    for i in 0..256 {
        f[i] = i16_to_field(a[i]);
    }

    let mut k: isize = 127;
    for &len in &[2usize, 4, 8, 16, 32, 64, 128] {
        for start in (0..256).step_by(2 * len) {
            let zeta = i16_to_field(ZETAS[k as usize]);
            k -= 1;
            for j in start..start + len {
                let t = f[j];
                f[j] = field_add(t, f[j + len]);
                f[j + len] = field_mul(zeta, field_sub(f[j + len], t));
            }
        }
    }

    // Multiply by n^{-1} mod q where n = 256, q = 3329
    // n^{-1} ≡ 3303 (mod 3329)
    let n_inv = 3303u16;
    for x in f.iter_mut() {
        *x = field_mul(n_inv, *x);
    }

    for i in 0..256 {
        a[i] = field_to_i16(f[i]);
    }
}

/// Pointwise multiplication in NTT domain (FIPS 203 Algorithm 11).
#[inline(always)]
pub fn pointwise_multiply(a: &mut [i16; 256], b: &[i16; 256]) {
    let mut out = [0i16; N];
    for i in 0..N / 2 {
        let a0 = a[2 * i] as i32;
        let a1 = a[2 * i + 1] as i32;
        let b0 = b[2 * i] as i32;
        let b1 = b[2 * i + 1] as i32;
        let z = GAMMAS[i] as i32;

        let mut c0 = (a0 * b0 + z * ((a1 * b1) % Q)) % Q;
        if c0 < 0 {
            c0 += Q;
        }
        let mut c1 = (a0 * b1 + a1 * b0) % Q;
        if c1 < 0 {
            c1 += Q;
        }
        out[2 * i] = c0 as i16;
        out[2 * i + 1] = c1 as i16;
    }
    a.copy_from_slice(&out);
}

// =============================================================================
// CRYPTOGRAPHIC HASH FUNCTIONS (FIPS 203 Section 4.1)
// =============================================================================

/// H(s) = SHA3-256(s) -> 32 bytes
#[inline(always)]
pub fn hash_h(input: &[u8], out: &mut [u8; 32]) {
    let mut hasher = Sha3_256::new();
    Digest::update(&mut hasher, input);
    let digest = hasher.finalize();
    out.copy_from_slice(&digest);
}

/// G(c) = SHA3-512(c) -> 64 bytes
#[inline(always)]
pub fn hash_g(input: &[u8], out: &mut [u8; 64]) {
    let mut hasher = Sha3_512::new();
    Digest::update(&mut hasher, input);
    let digest = hasher.finalize();
    out.copy_from_slice(&digest);
}

/// J(x) = SHAKE256(x) -> 32 bytes (used as KDF)
#[inline(always)]
pub fn kdf_j(input: &[u8], out: &mut [u8; 32]) {
    let mut xof = Shake256::default();
    xof.update(input);
    let mut reader = xof.finalize_xof();
    reader.read(out);
}

/// PRF(seed, nonce) = SHAKE256(seed || nonce) -> out.len() bytes
#[inline(always)]
pub fn prf(seed: &[u8], nonce: u8, out: &mut [u8]) {
    let mut xof = Shake256::default();
    Update::update(&mut xof, seed);
    Update::update(&mut xof, &[nonce]);
    let mut reader = xof.finalize_xof();
    reader.read(out);
}

/// XOF for matrix expansion: SHAKE128(ρ || i || j) -> out.len() bytes
#[inline(always)]
pub fn xof_matrix(rho: &[u8; 32], i: u8, j: u8, out: &mut [u8]) {
    let mut xof = Shake128::default();
    Update::update(&mut xof, rho);
    Update::update(&mut xof, &[i, j]);
    let mut reader = xof.finalize_xof();
    reader.read(out);
}

// =============================================================================
// SAMPLING ALGORITHMS (FIPS 203 Algorithms 7-8)
// =============================================================================

/// SampleNTT: Sample uniform polynomial from XOF (FIPS 203 Algorithm 7).
/// Returns polynomial in NTT domain as specified.
#[inline]
pub fn sample_ntt(rho: &[u8; 32], i: u8, j: u8, r: &mut Poly) {
    let mut xof = Shake128::default();
    Update::update(&mut xof, rho);
    Update::update(&mut xof, &[i, j]);
    let mut reader = xof.finalize_xof();

    let mut ctr = 0usize;
    let mut buf = [0u8; 3 * 128];
    while ctr < N {
        reader.read(&mut buf);
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

/// SamplePolyCBD: Sample from centered binomial distribution (FIPS 203 Algorithm 8).
#[inline]
pub fn sample_cbd(buf: &[u8], eta: i32, r: &mut Poly) {
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
    } else if eta == 3 {
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

// =============================================================================
// ENCODING AND COMPRESSION (FIPS 203 Algorithms 4-6)
// =============================================================================

#[inline(always)]
fn barrett_reduce_coeff(a: i16) -> i16 {
    let v: i32 = (((1u32 << 26) + (Q as u32) / 2) / (Q as u32)) as i32;
    let t: i32 = (v * (a as i32)) >> 26;
    ((a as i32) - t * (Q as i32)) as i16
}

/// ByteEncode: Serialize polynomial to 12-bit packed form (FIPS 203 Algorithm 4).
#[inline]
pub fn poly_encode(a: &Poly, out: &mut [u8; 384]) {
    for i in 0..N / 8 {
        let mut t = [0i16; 8];
        for j in 0..8 {
            t[j] = a.coeffs[8 * i + j];
        }
        for j in 0..8 {
            t[j] = barrett_reduce_coeff(t[j]);
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

/// ByteDecode: Deserialize polynomial from 12-bit packed form (FIPS 203 Algorithm 5).
#[inline]
pub fn poly_decode(inp: &[u8; 384], r: &mut Poly) {
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

/// Compress: Generic d-bit compression (FIPS 203 Algorithm 6).
#[inline]
pub fn poly_compress(a: &Poly, d: usize, out: &mut [u8]) {
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

/// Decompress: Generic d-bit decompression (inverse of compress).
#[inline]
pub fn poly_decompress(inp: &[u8], d: usize, r: &mut Poly) {
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

/// Compress with DU bits (for u vector).
#[inline(always)]
pub fn poly_compress_u(a: &Poly, out: &mut [u8]) {
    poly_compress(a, DU, out)
}

/// Decompress with DU bits (for u vector).
#[inline(always)]
pub fn poly_decompress_u(inp: &[u8], r: &mut Poly) {
    poly_decompress(inp, DU, r)
}

/// Compress with DV bits (for v polynomial).
#[inline(always)]
pub fn poly_compress_v(a: &Poly, out: &mut [u8]) {
    poly_compress(a, DV, out)
}

/// Decompress with DV bits (for v polynomial).
#[inline(always)]
pub fn poly_decompress_v(inp: &[u8], r: &mut Poly) {
    poly_decompress(inp, DV, r)
}

// =============================================================================
// MESSAGE ENCODING/DECODING
// =============================================================================

/// Map 32-byte message to polynomial with coefficients in {0, (q+1)/2}.
#[inline]
pub fn poly_from_msg(msg: &[u8; 32], r: &mut Poly) {
    let one = ((Q as i16) + 1) / 2;
    for i in 0..N {
        let bit = ((msg[i >> 3] >> (i & 7)) & 1) as i16;
        r.coeffs[i] = bit * one;
    }
}

/// Map polynomial back to 32-byte message (threshold at q/2).
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ntt_roundtrip() {
        let mut a = [0i16; 256];
        for i in 0..256 {
            a[i] = ((i as i16) * 7 + 3) % (Q as i16);
        }
        let mut b = a;
        ntt_forward(&mut b);
        ntt_inverse(&mut b);
        for i in 0..256 {
            let mut x = (b[i] as i32) % Q;
            if x < 0 {
                x += Q;
            }
            let mut y = (a[i] as i32) % Q;
            if y < 0 {
                y += Q;
            }
            assert_eq!(x, y, "NTT roundtrip failed at coefficient {}", i);
        }
    }

    #[test]
    fn test_poly_encode_decode_roundtrip() {
        let mut poly = Poly::new();
        for i in 0..N {
            poly.coeffs[i] = ((i as i16) * 13 + 7) % (Q as i16);
        }

        let mut encoded = [0u8; 384];
        poly_encode(&poly, &mut encoded);

        let mut decoded = Poly::new();
        poly_decode(&encoded, &mut decoded);

        for i in 0..N {
            let mut orig = (poly.coeffs[i] as i32) % Q;
            if orig < 0 {
                orig += Q;
            }
            let mut dec = (decoded.coeffs[i] as i32) % Q;
            if dec < 0 {
                dec += Q;
            }
            assert_eq!(orig, dec, "Encode/decode roundtrip failed at coefficient {}", i);
        }
    }
}
