#![cfg(feature = "serde")]

use serde_json::{to_vec, from_slice};

use rusty_kyber::api::{PublicKey, SecretKey, Ciphertext, SharedSecret};
use rusty_kyber::params::{PUBLIC_KEY_BYTES, SECRET_KEY_BYTES, CIPHERTEXT_BYTES, SHARED_SECRET_BYTES};

#[test]
fn serde_roundtrip_public_types() {
    let pk = PublicKey([0x11u8; PUBLIC_KEY_BYTES]);
    let sk = SecretKey([0x22u8; SECRET_KEY_BYTES]);
    let ct = Ciphertext([0x33u8; CIPHERTEXT_BYTES]);
    let ss = SharedSecret([0x44u8; SHARED_SECRET_BYTES]);

    let pk2: PublicKey = from_slice(&to_vec(&pk).unwrap()).unwrap();
    let sk2: SecretKey = from_slice(&to_vec(&sk).unwrap()).unwrap();
    let ct2: Ciphertext = from_slice(&to_vec(&ct).unwrap()).unwrap();
    let ss2: SharedSecret = from_slice(&to_vec(&ss).unwrap()).unwrap();

    assert_eq!(pk.as_bytes(), pk2.as_bytes());
    assert_eq!(sk.as_bytes(), sk2.as_bytes());
    assert_eq!(ct.as_bytes(), ct2.as_bytes());
    assert_eq!(ss.as_bytes(), ss2.as_bytes());
}
