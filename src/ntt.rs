//! In-place Kyber NTT and inverse NTT over R_q with q = 3329.
//!
//! Implementation uses the standard zeta schedule and integrates Montgomery and
//! Barrett reductions to keep coefficients bounded. Both transforms are
//! branch-free with fixed memory access patterns.

use crate::params::Q;

#[inline(always)]
fn montgomery_reduce(a: i32) -> i16 {
    // R = 2^16, qinv = -q^{-1} mod 2^16 = 62209
    let u = (a as i64 * 62209i64) as i32 & 0xFFFF;
    let t = (a - u * Q) >> 16;
    t as i16
}

#[inline(always)]
fn fqmul(a: i16, b: i16) -> i16 {
    montgomery_reduce((a as i32) * (b as i32))
}

#[inline(always)]
fn barrett_reduce(mut a: i16) -> i16 {
    // Barrett with v = floor(2^26 / q + 0.5)
    let v = ((1u32 << 26) + (Q as u32 / 2)) / (Q as u32);
    let t = ((v as u32 * (a as u32)) >> 26) as i16;
    a -= t * (Q as i16);
    a
}

/// Forward NTT (in place).
#[inline(always)]
pub fn ntt(a: &mut [i16; 256]) {
    let mut len = 128usize;
    let mut k = 0usize;

    while len >= 2 {
        let mut start = 0usize;
        while start < 256 {
            let zeta = ZETAS[k];
            k += 1;
            for j in start..start + len {
                let t = fqmul(zeta, a[j + len]);
                let u = a[j];
                a[j] = barrett_reduce(u + t);
                a[j + len] = barrett_reduce(u - t);
            }
            start += 2 * len;
        }
        len >>= 1;
    }
}

/// Inverse NTT (in place), including multiplication by n^{-1}.
#[inline(always)]
pub fn inv_ntt(a: &mut [i16; 256]) {
    let mut len = 2usize;
    let mut k = 127isize;

    while len <= 128 {
        let mut start = 0usize;
        while start < 256 {
            let zeta = -ZETAS[k as usize];
            k -= 1;
            for j in start..start + len {
                let u = a[j];
                let v = a[j + len];
                a[j] = barrett_reduce(u + v);
                let t = u - v;
                a[j + len] = fqmul(zeta, t);
            }
            start += 2 * len;
        }
        len <<= 1;
    }

    // Multiply by n^{-1} in Montgomery domain: 1441
    for x in a.iter_mut() {
        *x = fqmul(*x, 1441);
    }
}

/// Kyber zetas (twiddle factors) for the NTT.
pub const ZETAS: [i16; 128] = [
    -1044, -758,  -359, -1517, 1493, 1422, 287,  202,  -171, 622,  1577, 182,  962,  -1202, -1474,
    1468,  573,   -1325, 264,  383,  -829, 1458, -1602, -130, -681, 1017, 732,  608,  -1542, 411,
    -205,  -1571, 1223, 652,  -552, 1015, -1293, 1491, -282, -1544, 516,  -8,   -320, -666,  -1618,
    -1162, 126,   1469, -853, -90,  -271, 830,  107,  -1421, -247, -951, -398, 961,  -1508, -725,
    448,   -1065, 677,  -1275, -1103, 430,  555,  843,  -1251, 871,  1550, 105,  422,  587,   177,
    -235,  -291,  -460, 1574, 1653, -246,  778,  1159, -147,  -777, 1483, -602, 1119, -1590, 644,
    -872,  349,   418,  329,  -156, -75,   817,  1097, 603,  610,  1322, -1285, -1465, 384,   -1215,
    -136,  1218,  -1335, -874, 220,  -1187, -1659, -1185, -1530, -1278, 794,  -1510, -854,  -870,
    478,   -108,  -308, 996,  991,  958,  -1460, 1522, 1628,
];
