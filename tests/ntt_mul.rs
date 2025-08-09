use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use rand_core::RngCore;

use rusty_kyber::params::{N, Q};
use rusty_kyber::poly::Poly;

fn canon_q(x: i32) -> i16 {
    let mut v = x % Q;
    if v < 0 {
        v += Q;
    }
    v as i16
}

fn ref_mul(a: &Poly, b: &Poly) -> Poly {
    // Reference schoolbook convolution mod (X^256 + 1) over Z_q.
    let mut c = [0i32; N * 2 - 1];
    for i in 0..N {
        for j in 0..N {
            c[i + j] += (a.coeffs[i] as i32) * (b.coeffs[j] as i32);
        }
    }
    // Reduce mod X^256 + 1
    for k in (N..(2 * N - 1)).rev() {
        c[k - N] -= c[k];
    }
    // Map to [0,q)
    let mut out = Poly::new();
    for i in 0..N {
        out.coeffs[i] = canon_q(c[i]);
    }
    out
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
fn ntt_pointwise_matches_reference_mul() {
    for s in 0u8..6 {
        let mut a = rand_poly([0xA0 + s; 32]);
        let mut b = rand_poly([0xB0 + s; 32]);

        // Reference time-domain multiplication
        let cref = ref_mul(&a, &b);

        // NTT pipeline: NTT(a), NTT(b), pointwise, InvNTT
        a.ntt();
        b.ntt();
        let mut c = a;
        c.pointwise_mul(&b);
        c.inv_ntt();

        for i in 0..N {
            assert_eq!(cref.coeffs[i] as i32 % Q, c.coeffs[i] as i32 % Q, "i={}", i);
        }
    }
}
