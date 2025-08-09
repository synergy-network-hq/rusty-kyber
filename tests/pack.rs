use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

use rusty_kyber::params::{CIPHERTEXT_BYTES, DU, DV, N, Q};
use rusty_kyber::poly::Poly;
use rusty_kyber::utils::{
    poly_compress_d, poly_decompress_d,
    poly_compress_u, poly_decompress_u,
    poly_compress_v, poly_decompress_v,
    poly_from_bytes, poly_to_bytes,
};

fn canon_q(x: i16) -> i32 {
    let mut v = x as i32 % Q;
    if v < 0 { v += Q; }
    v
}

fn rand_poly(seed: [u8; 32]) -> Poly {
    let mut rng = ChaCha20Rng::from_seed(seed);
    let mut p = Poly::new();
    for i in 0..N {
        // uniform-ish in 0..Q
        let v = (rng.next_u32() as i32) % Q;
        p.coeffs[i] = v as i16;
    }
    p
}

#[test]
fn poly_12bit_pack_roundtrip() {
    // Several seeds to exercise variation
    for s in 0u8..8 {
        let mut a = rand_poly([s; 32]);

        // 12-bit codec used by Kyber's polynomial packer
        let mut bytes = [0u8; 384];
        poly_to_bytes(&a, &mut bytes);

        let mut b = Poly::new();
        poly_from_bytes(&bytes, &mut b);

        for i in 0..N {
            assert_eq!(canon_q(a.coeffs[i]), canon_q(b.coeffs[i]), "coeff {}", i);
        }

        // Also check stability after canonicalization
        let mut bytes2 = [0u8; 384];
        poly_to_bytes(&b, &mut bytes2);
        assert_eq!(bytes, bytes2, "re-encode differs");
    }
}

#[test]
fn d_bit_codecs_cover_all_required_widths() {
    // The spec-related widths we care about: 4/5/10/11/12/13 bits
    let ds = [4usize, 5, 10, 11, 12, 13];

    for (idx, d) in ds.iter().enumerate() {
        let a = rand_poly([0xA0 + idx as u8; 32]);

        let mut enc = vec![0u8; d * N / 8];
        poly_compress_d(&a, *d, &mut enc);

        let mut b = Poly::new();
        poly_decompress_d(&enc, *d, &mut b);

        // Compressing again should be identical (idempotence)
        let mut enc2 = vec![0u8; d * N / 8];
        poly_compress_d(&b, *d, &mut enc2);
        assert_eq!(enc, enc2, "idempotence failed for d={}", d);
    }
}

#[test]
fn du_dv_wrappers_match_sizes() {
    let a = rand_poly([0x55; 32]);

    // DU
    let mut cu = vec![0u8; DU * N / 8];
    poly_compress_u(&a, &mut cu);
    let mut au = Poly::new();
    poly_decompress_u(&cu, &mut au);
    let mut cu2 = vec![0u8; DU * N / 8];
    poly_compress_u(&au, &mut cu2);
    assert_eq!(cu, cu2, "DU wrapper not idempotent");

    // DV
    let mut cv = vec![0u8; DV * N / 8];
    poly_compress_v(&a, &mut cv);
    let mut av = Poly::new();
    poly_decompress_v(&cv, &mut av);
    let mut cv2 = vec![0u8; DV * N / 8];
    poly_compress_v(&av, &mut cv2);
    assert_eq!(cv, cv2, "DV wrapper not idempotent");
}

#[test]
fn ciphertext_size_consistency() {
    // Sanity: computed CT size equals our DU/DV totals for active K
    let expected = (DU * N / 8) * rusty_kyber::params::K + (DV * N / 8);
    assert_eq!(CIPHERTEXT_BYTES, expected);
}
