use rusty_kyber::params::{N, Q};
use rusty_kyber::poly::Poly;
use rusty_kyber::utils::{poly_from_msg, poly_to_msg};

fn canon_q(x: i16) -> i32 {
    let mut v = x as i32 % Q;
    if v < 0 { v += Q; }
    v
}

#[test]
fn msg_poly_roundtrip_randomish() {
    // Deterministic inputs; not using RNG to keep test hermetic
    let vectors = [
        [0u8; 32],
        [0xFF; 32],
        [0xAA; 32],
        [0x55; 32],
        [0x00, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
               16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31],
    ];

    for m in vectors {
        let mut p = Poly::new();
        poly_from_msg(&m, &mut p);

        // Check coefficients are exactly in {0, (q+1)/2}
        let half = ((Q as i16) + 1) / 2;
        for i in 0..N {
            let v = canon_q(p.coeffs[i]) as i16;
            assert!(v == 0 || v == half, "coeff {} has unexpected value {}", i, v);
        }

        let mut out = [0u8; 32];
        poly_to_msg(&p, &mut out);
        assert_eq!(out, m, "roundtrip mismatch");
    }
}
