use rusty_kyber::params::N;
use rusty_kyber::poly::Poly;
use rusty_kyber::utils::{cbd, cbd_eta};

fn in_range(v: i16, eta: i16) -> bool {
    v >= -eta && v <= eta
}

#[test]
fn cbd_eta2_bounds_and_alias() {
    // eta=2 uses 4 bytes per 8 coeffs => N/8 blocks
    let mut buf = vec![0u8; (N / 8) * 4];
    // Deterministic but non-trivial pattern
    for (i, b) in buf.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(0x9d).wrapping_add(0x37);
    }

    let mut r1 = Poly::new();
    cbd_eta(&buf, 2, &mut r1);

    // Bounds check
    for &c in r1.coeffs.iter() {
        assert!(in_range(c, 2), "eta=2 bound violated: {}", c);
    }

    // Alias cbd() equals cbd_eta(..., 2)
    let mut r2 = Poly::new();
    cbd(&buf, &mut r2);
    assert_eq!(&r1.coeffs[..], &r2.coeffs[..]);
}

#[test]
fn cbd_eta3_bounds() {
    // eta=3 uses 3 bytes per 4 coeffs => N/4 blocks
    let mut buf = vec![0u8; (N / 4) * 3];
    for (i, b) in buf.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(0x5b).wrapping_add(0x21);
    }

    let mut r = Poly::new();
    cbd_eta(&buf, 3, &mut r);

    for &c in r.coeffs.iter() {
        assert!(in_range(c, 3), "eta=3 bound violated: {}", c);
    }
}
