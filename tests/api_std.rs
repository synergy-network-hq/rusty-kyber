#![cfg(feature = "std")]

use rusty_kyber::{decapsulate, encapsulate_osrng, keypair_osrng};

#[test]
fn api_osrng_roundtrip() {
    let (pk, sk) = keypair_osrng();
    let (ct, ss1) = encapsulate_osrng(&pk);
    let ss2 = decapsulate(&sk, &ct);

    assert_eq!(ss1.as_bytes(), ss2.as_bytes());
}
