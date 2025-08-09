use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

use rusty_kyber::{keypair, encapsulate, decapsulate};

#[test]
fn decaps_mismatch_on_corrupted_ciphertext() {
    // Deterministic RNGs
    let mut rng_kp = ChaCha20Rng::from_seed([0xA5; 32]);
    let mut rng_ct = ChaCha20Rng::from_seed([0x5A; 32]);

    let (pk, sk) = keypair(&mut rng_kp);
    let (ct, ss_sender) = encapsulate(&mut rng_ct, &pk);

    // Corrupt a copy of the ciphertext
    let mut ct_bad = ct.into_bytes();
    for (i, b) in ct_bad.iter_mut().enumerate() {
        *b ^= ((i as u8) ^ 0x5A) & 0x07;
        if i > 7 { break; } // small, deterministic corruption
    }
    let ct_bad = rusty_kyber::api::Ciphertext(ct_bad);

    let ss_receiver = decapsulate(&sk, &ct_bad);
    assert_ne!(ss_sender.as_bytes(), ss_receiver.as_bytes());
}
