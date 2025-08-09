#![no_main]
use libfuzzer_sys::fuzz_target;

use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

use rusty_kyber::{decapsulate, encapsulate, keypair};

fn seed32(data: &[u8], pad: u8) -> [u8; 32] {
    let mut s = [pad; 32];
    let n = core::cmp::min(32, data.len());
    s[..n].copy_from_slice(&data[..n]);
    s
}

fuzz_target!(|data: &[u8]| {
    // Two deterministic RNGs from fuzzer input so we exercise encaps randomness
    let mut rng_kp = ChaCha20Rng::from_seed(seed32(data, 0xA5));
    let mut rng_ct = ChaCha20Rng::from_seed(seed32(data, 0x5A));

    // Round-trip must hold
    let (pk, sk) = keypair(&mut rng_kp);
    let (ct, ss1) = encapsulate(&mut rng_ct, &pk);
    let ss2 = decapsulate(&sk, &ct);
    assert_eq!(ss1.as_bytes(), ss2.as_bytes());

    // Optional adversarial poke: mutate ct with fuzzer bytes (no panics allowed)
    if !data.is_empty() {
        let mut ct_bad = ct.into_bytes();
        for (i, b) in data.iter().enumerate() {
            ct_bad[i % ct_bad.len()] ^= b / 7; // light-touch corrupt
        }
        let _ = decapsulate(&sk, &ct_bad.into());
        // No assert: correctness isn't guaranteed for corrupted ciphertexts, but it must not panic.
    }
});
