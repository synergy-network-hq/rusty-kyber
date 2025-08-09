# rusty-kyber

Pure-Rust implementation of **ML-KEM (Kyber)** with constant-time core ops, full NIST KAT coverage, and `no_std` support.

[![docs.rs](https://img.shields.io/docsrs/rusty-kyber)](https://docs.rs/rusty-kyber)
[![Crates.io](https://img.shields.io/crates/v/rusty-kyber.svg)](https://crates.io/crates/rusty-kyber)
![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)

## Features

- Pure Rust, `#![forbid(unsafe_code)]`
- Constant-time decapsulation path (verified with `subtle`)
- **Levels**: `kyber512` (default), `kyber768`, `kyber1024`
- `std` (default) or `no_std`
- Optional: `serde` for API types; `zeroize` for key erasure; `hardening` for redundant checks
- NIST **KAT** harness (byte-for-byte `pk/sk/ct/ss`), plus fuzz targets
- Benches for NTT, poly ops, and end-to-end KEM

## Quick start

```toml
# Cargo.toml
[dependencies]
rusty-kyber = "0.1"
```

```rust
use rand::rngs::OsRng;
use rusty_kyber::{keypair, encapsulate, decapsulate};

let (pk, sk) = keypair(&mut OsRng);
let (ct, ss_sender) = encapsulate(&mut OsRng, &pk);
let ss_receiver = decapsulate(&sk, &ct);
assert_eq!(ss_sender.as_bytes(), ss_receiver.as_bytes());
```

## Feature flags

- Levels (select one): kyber512 (default), kyber768, kyber1024

- Platform: std (default; enables OsRng), or build with --no-default-features for no_std

- Optional:

    - serde — serialize/deserialize PublicKey, SecretKey, Ciphertext, SharedSecret

    - zeroize — wipe SecretKey on drop

    - hardening — redundant constant-time checks and scrubbing in sensitive paths

## Examples:

    ```bash
    # kyber768 with std
    cargo test --no-default-features --features "std,kyber768"

    # no_std build (wasm32)
    rustup target add wasm32-unknown-unknown
    cargo build --target wasm32-unknown-unknown --no-default-features --features kyber512
    ```

## Security

See SECURITY.md for the threat model and constant-time notes.

- Default decapsulation is constant-time with re-encrypt verify.

- Enable hardening to add double-execution and buffer scrubbing (requires zeroize).

## Testing
- KATs: end-to-end tests for all levels under tests/.

- Fuzz: cargo +nightly fuzz run pack_unpack / kem_roundtrip.

- Benches: cargo bench (Criterion).

## MSRV
rust-version = 1.65 (documented in Cargo.toml). MSRV bumps are considered minor-version changes under SemVer.

## License
Licensed under either of

- Apache License, Version 2.0

- MIT license

