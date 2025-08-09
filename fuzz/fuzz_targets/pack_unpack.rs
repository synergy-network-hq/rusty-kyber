#![no_main]
use libfuzzer_sys::fuzz_target;

use rusty_kyber::params::{CIPHERTEXT_BYTES, DU, DV, N, Q, K};
use rusty_kyber::poly::Poly;
use rusty_kyber::utils::{
    poly_compress_u, poly_compress_v, poly_decompress_u, poly_decompress_v,
    poly_from_bytes, poly_to_bytes,
};

fn canon_q(x: i16) -> i32 {
    let mut v = x as i32 % Q;
    if v < 0 { v += Q; }
    v
}

fuzz_target!(|data: &[u8]| {
    // Build a polynomial from fuzzer data (deterministic mapping).
    let mut a = Poly::new();
    for i in 0..N {
        let b0 = *data.get(2*i).unwrap_or(&0);
        let b1 = *data.get(2*i + 1).unwrap_or(&0);
        let v = u16::from_le_bytes([b0, b1]) as i32;
        a.coeffs[i] = (v % Q) as i16;
    }

    // --- 12-bit pack/unpack must be lossless modulo q canonicalization ---
    let mut bytes = [0u8; 384];
    poly_to_bytes(&a, &mut bytes);

    let mut b = Poly::new();
    poly_from_bytes(&bytes, &mut b);

    for i in 0..N {
        assert_eq!(canon_q(a.coeffs[i]), canon_q(b.coeffs[i]));
    }

    // --- DU codec: compress -> decompress -> compress is idempotent ---
    let mut cu = vec![0u8; DU * N / 8];
    poly_compress_u(&a, &mut cu);
    let mut au = Poly::new();
    poly_decompress_u(&cu, &mut au);
    let mut cu2 = vec![0u8; DU * N / 8];
    poly_compress_u(&au, &mut cu2);
    assert_eq!(cu, cu2);

    // --- DV codec: same property ---
    let mut cv = vec![0u8; DV * N / 8];
    poly_compress_v(&a, &mut cv);
    let mut av = Poly::new();
    poly_decompress_v(&cv, &mut av);
    let mut cv2 = vec![0u8; DV * N / 8];
    poly_compress_v(&av, &mut cv2);
    assert_eq!(cv, cv2);

    // Sanity: ciphertext-size constant matches our DU/DV totals
    let expected_ct_bytes = (DU * N / 8) * K + (DV * N / 8);
    assert_eq!(CIPHERTEXT_BYTES, expected_ct_bytes);
});
