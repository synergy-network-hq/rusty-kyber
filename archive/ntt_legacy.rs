//! In-place Kyber NTT and inverse NTT over R_q with q = 3329.
//! Pure field-arithmetic variant (no Montgomery form).
//! Zetas and butterfly schedule match ML-KEM (Kyber) reference exactly.

use crate::params::Q;

#[inline(always)]
fn small_reduce(x: u16) -> u16 {
    if x < (Q as u16) { x } else { x - (Q as u16) }
}

#[inline(always)]
fn barrett_reduce(x: u32) -> u16 {
    // Barrett reduction with multiplier 2^24 / q.
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

/// Forward NTT (matches ML-KEM twiddle schedule).
#[inline(always)]
pub fn ntt(a: &mut [i16; 256]) {
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

#[test]
fn ntt_inv_roundtrip_modq() {
    let mut a = [0i16; 256];
    for i in 0..256 {
        a[i] = ((i as i16) * 7 + 3) % (Q as i16);
    }
    let mut b = a;
    ntt(&mut b);
    inv_ntt(&mut b);
    for i in 0..256 {
        let mut x = (b[i] as i32) % Q;
        if x < 0 {
            x += Q;
        }
        let mut y = (a[i] as i32) % Q;
        if y < 0 {
            y += Q;
        }
        assert_eq!(x, y, "mismatch at {}", i);
    }
}

/// Inverse NTT (uses −zeta) and multiplies by n^{-1} mod q (n = 256).
#[inline(always)]
pub fn inv_ntt(a: &mut [i16; 256]) {
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

    // n^{-1} mod q for n=256 and q=3329 is 3303 (matches ml-kem).
    let n_inv = 3303u16;
    for x in f.iter_mut() {
        *x = field_mul(n_inv, *x);
    }

    for i in 0..256 {
        a[i] = field_to_i16(f[i]);
    }
}

/// Zeta table from FIPS 203 Appendix A (official ML-KEM standard)
/// These are the values of ζ^BitRev₇(i) mod q for i ∈ {0, ..., 127}
pub const ZETAS: [i16; 128] = [
    1, 1729, 2580, 3289, 2642, 630, 1897, 848, 1062, 1919, 193, 797, 2786, 3260, 569, 1746, 296, 2447,
    1339, 1476, 3046, 56, 2240, 1333, 1426, 2094, 535, 2882, 2393, 2879, 1974, 821, 289, 331, 3253, 1756,
    1197, 2304, 2277, 2055, 650, 1977, 2513, 632, 2865, 33, 1320, 1915, 2319, 1435, 807, 452, 1438, 2868,
    1534, 2402, 2647, 2617, 1481, 648, 2474, 3110, 1227, 910, 17, 2761, 583, 2649, 1637, 723, 2288, 1100,
    1409, 2662, 3281, 233, 756, 2156, 3015, 3050, 1703, 1651, 2789, 1789, 1847, 952, 1461, 2687,
    939, 2308, 2437, 2388, 733, 2337, 268, 641, 1584, 2298, 2037, 3220, 375, 2549, 2090, 1645,
    1063, 319, 2773, 757, 2099, 561, 2466, 2594, 2804, 1092, 403, 1026, 1143, 2150, 2775, 886,
    1722, 1212, 1874, 1029, 2110, 2935, 885, 2154,
];

/// Gamma table (ζ^{2·BitRev7(i)+1} mod q for i∈0..127) used by base-case NTT multiplication
pub const GAMMAS: [i16; 128] = [
    17, 2761, 583, 2649, 1637, 723, 2288, 1100, 1409, 2662, 3281, 233, 756, 2156, 3015, 3050, 1703, 1651,
    2789, 1789, 1847, 952, 1461, 2687, 939, 2308, 2437, 2388, 733, 2337, 268, 641, 1584, 2298,
    2037, 3220, 375, 2549, 2090, 1645, 1063, 319, 2773, 757, 2099, 561, 2466, 2594, 2804, 1092,
    403, 1026, 1143, 2150, 2775, 886, 1722, 1212, 1874, 1029, 2110, 2935, 885, 2154, 17, 2761, 583, 2649,
    1637, 723, 2288, 1100, 1409, 2662, 3281, 233, 756, 2156, 3015, 3050, 1703, 1651, 2789, 1789,
    1847, 952, 1461, 2687, 939, 2308, 2437, 2388, 733, 2337, 268, 641, 1584, 2298, 2037, 3220, 375, 2549,
    2090, 1645, 1063, 319, 2773, 757, 2099, 561, 2466, 2594, 2804, 1092, 403, 1026, 1143, 2150,
    2775, 886, 1722, 1212, 1874, 1029, 2110, 2935, 885, 2154,
];
