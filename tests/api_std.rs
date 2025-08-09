#![cfg(feature = "std")]

use rand::rngs::OsRng;
use rusty_kyber::{keypair_osrng, encapsulate_osrng, decapsulate};

#[test]
fn api_osrng_roundtrip() {
    let mut rng = OsRng;
    let (pk, sk) = keypair_osrng(&mut rng);
    let (ct, ss1) = encapsulate_osrng(&mut rng, &pk);
    let ss2 = decapsulate(&sk, &ct);
    assert_eq!(ss1.as_bytes(), ss2.as_bytes());
}
