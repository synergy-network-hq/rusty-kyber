#![cfg(feature = "hardening")]

use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

use rusty_kyber::{keypair, encapsulate, decapsulate};
use rusty_kyber::hardening::decapsulate_hardened;

#[test]
fn hardened_equals_normal_on_valid_ciphertext() {
    let mut rng = ChaCha20Rng::from_seed([0x7A; 32]);
    let (pk, sk) = keypair(&mut rng);
    let (ct, ss1) = encapsulate(&mut rng, &pk);

    let ss_normal = decapsulate(&sk, &ct);
    let ss_hard = decapsulate_hardened(&sk, &ct);

    assert_eq!(ss1.as_bytes(), ss_normal.as_bytes());
    assert_eq!(ss1.as_bytes(), ss_hard.as_bytes());
}
