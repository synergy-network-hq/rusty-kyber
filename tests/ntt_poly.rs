use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use rand_core::RngCore;

use rusty_kyber::params::{N, Q};
use rusty_kyber::poly::Poly;

fn canon_q(x: i16) -> i32 {
    let mut v = x as i32 % Q;
    if v < 0 {
        v += Q;
    }
    v
}

fn rand_poly(seed: [u8; 32]) -> Poly {
    let mut rng = ChaCha20Rng::from_seed(seed);
    let mut p = Poly::new();
    for i in 0..N {
        let v = (rng.next_u32() as i32) % Q;
        p.coeffs[i] = v as i16;
    }
    p
}

#[test]
fn ntt_invntt_roundtrip() {
    for s in 0u8..8 {
        let mut a = rand_poly([s; 32]);
        let original = a;

        // Forward + inverse should recover the same coefficients modulo q
        a.ntt();
        a.inv_ntt();

        for i in 0..N {
            assert_eq!(canon_q(a.coeffs[i]), canon_q(original.coeffs[i]), "i={}", i);
        }
    }
}

#[test]
fn poly_add_sub_inverses() {
    for s in 0u8..8 {
        let a = rand_poly([s; 32]);
        let b = rand_poly([0xA5 ^ s; 32]);

        let mut c = a;
        c.add(&b);
        c.sub(&b);

        for i in 0..N {
            assert_eq!(canon_q(c.coeffs[i]), canon_q(a.coeffs[i]), "i={}", i);
        }
    }
}

// Reference implementation from ml-kem, adapted for i16
#[inline(always)]
fn fe_add_ref(a: u16, b: u16) -> u16 {
    (a + b) % (Q as u16)
}

#[inline(always)]
fn fe_sub_ref(a: u16, b: u16) -> u16 {
    (a + (Q as u16) - b) % (Q as u16)
}

#[inline(always)]
fn fe_mul_ref(a: u16, b: u16) -> u16 {
    ((a as u32) * (b as u32) % (Q as u32)) as u16
}

pub fn ntt_ref(a: &mut [i16; 256]) {
    let mut f = [0u16; 256];
    for i in 0..256 {
        f[i] = a[i] as u16;
    }

    let mut k = 1;
    let zetas = rusty_kyber::ntt::ZETAS;
    for &len in &[128, 64, 32, 16, 8, 4, 2] {
        for start in (0..256).step_by(2 * len) {
            let zeta = zetas[k] as u16;
            k += 1;
            for j in start..(start + len) {
                let t = fe_mul_ref(zeta, f[j + len]);
                f[j + len] = fe_sub_ref(f[j], t);
                f[j] = fe_add_ref(f[j], t);
            }
        }
    }

    for i in 0..256 {
        a[i] = f[i] as i16;
    }
}

#[test]
fn compare_ntt_implementations() {
    let mut rng = ChaCha20Rng::from_seed([0; 32]);
    let mut a = Poly::new();
    let mut b = Poly::new();

    for i in 0..N {
        let val = (rng.next_u32() % (Q as u32)) as i16;
        a.coeffs[i] = val;
        b.coeffs[i] = val;
    }

    a.ntt();
    ntt_ref(&mut b.coeffs);

    for i in 0..N {
        assert_eq!(canon_q(a.coeffs[i]), canon_q(b.coeffs[i]), "NTT implementation mismatch at index {}", i);
    }
}
