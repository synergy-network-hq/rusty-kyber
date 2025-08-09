#![cfg(feature = "hardening")]

use subtle::{Choice, ConditionallySelectable};
use rand::{SeedableRng};
use rand_chacha::ChaCha20Rng;

use rusty_kyber::{keypair};
use rusty_kyber::api::Ciphertext;
use rusty_kyber::hardening::decapsulate_hardened;
use rusty_kyber::encapsulate;

#[test]
fn hardening_returns_zero_on_fault() {
    let mut rng_kp = ChaCha20Rng::from_seed([0x23; 32]);
    let mut rng_ct = ChaCha20Rng::from_seed([0x42; 32]);

    let (pk, sk) = keypair(&mut rng_kp);
    let (ct_good, _ss_sender) = encapsulate(&mut rng_ct, &pk);

    // Corrupt ciphertext
    let mut ct_bad = ct_good.into_bytes();
    ct_bad[0] ^= 0xFF;
    let ct_bad = Ciphertext(ct_bad);

    let ss = decapsulate_hardened(&sk, &ct_bad);
    assert_eq!(ss.as_bytes(), &[0u8; 32]);
}
