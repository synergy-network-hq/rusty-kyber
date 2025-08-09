use rusty_kyber::params::{
    CIPHERTEXT_BYTES, DU, DV, K, N, POLY_BYTES, PUBLIC_KEY_BYTES, Q, SECRET_KEY_BYTES,
};

#[test]
fn public_key_size_formula() {
    // Kyber public key: rho (32) || t (K polynomials, 384 each)
    assert_eq!(PUBLIC_KEY_BYTES, 32 + K * POLY_BYTES);
}

#[test]
fn ciphertext_size_formula() {
    // u (K * DU*N/8) || v (DV*N/8)
    let expected = (DU * N / 8) * K + (DV * N / 8);
    assert_eq!(CIPHERTEXT_BYTES, expected);
}

#[test]
fn secret_key_sane() {
    // Sanity: secret key must be larger than public key and ciphertext (Kyber structure)
    assert!(SECRET_KEY_BYTES > PUBLIC_KEY_BYTES);
    assert!(SECRET_KEY_BYTES > CIPHERTEXT_BYTES);
}

#[test]
fn modulus_basic_properties() {
    assert!(Q > 0 && Q < (1 << 15)); // fits i16 domain as used
}
