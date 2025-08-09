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
        let mut a = rand_poly([s; 32]);
        let b = rand_poly([0xA5 ^ s; 32]);

        let mut c = a;
        c.add(&b);
        c.sub(&b);

        for i in 0..N {
            assert_eq!(canon_q(c.coeffs[i]), canon_q(a.coeffs[i]), "i={}", i);
        }
    }
}
