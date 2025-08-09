#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
//! # rusty-kyber
//!
//! Pure-Rust ML-KEM (Kyber) implementation with constant-time core ops,
//! feature-gated parameter sets, and full NIST KAT harnesses in `tests/`.
//!
//! ## Feature flags
//! - **Levels** (select one): `kyber512` *(default)*, `kyber768`, `kyber1024`
//! - **Platform**: `std` *(default; enables OsRng)*, or `no_std`
//! - **Optional**: `serde` (serialize public types), `zeroize` (zeroize `SecretKey` on drop),
//!   `hardening` (redundant constant-time checks and buffer scrubbing)

pub mod params;
pub mod ntt;
pub mod poly;
pub mod utils;
pub mod keygen;
pub mod encaps;
pub mod decaps;
pub mod kem;
pub mod api;

#[cfg(feature = "hardening")]
#[cfg_attr(docsrs, doc(cfg(feature = "hardening")))]
pub mod hardening;

// Re-export the ergonomic API at the crate root
pub use crate::api::{
    decapsulate, encapsulate, keypair, Ciphertext, PublicKey, SecretKey, SharedSecret,
};

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use crate::api::{encapsulate_osrng, keypair_osrng};

#[cfg(feature = "hardening")]
#[cfg_attr(docsrs, doc(cfg(feature = "hardening")))]
pub use crate::hardening::decapsulate_hardened;
