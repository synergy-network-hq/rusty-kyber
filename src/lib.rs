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

// FIPS 203 ML-KEM Implementation Modules
pub mod params; // Parameter sets and constants (FIPS 203 Table 2)
pub mod math; // Consolidated mathematical operations (FIPS 203 Algorithms 4-11)

// Legacy modules moved to archive/ folder
// pub mod ntt; // Legacy NTT implementation - moved to archive/ntt_legacy.rs
// pub mod poly; // Legacy polynomial operations - moved to archive/poly_legacy.rs
// pub mod utils; // Legacy utility functions - moved to archive/utils_legacy.rs

// Core ML-KEM algorithms (FIPS 203 Algorithms 12-15)
pub mod keygen; // ML-KEM.KeyGen (Algorithm 15)
pub mod encaps; // ML-KEM.Encaps (Algorithm 16)
pub mod decaps; // ML-KEM.Decaps (Algorithm 17)
pub mod kem; // High-level KEM interface
pub mod api; // Public API and type wrappers

#[cfg(feature = "hardening")]
#[cfg_attr(docsrs, doc(cfg(feature = "hardening")))]
pub mod hardening;

// Re-export the ergonomic API at the crate root
pub use crate::api::{
    decapsulate,
    encapsulate,
    keypair,
    Ciphertext,
    PublicKey,
    SecretKey,
    SharedSecret,
};

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use crate::api::{ encapsulate_osrng, keypair_osrng };

#[cfg(feature = "hardening")]
#[cfg_attr(docsrs, doc(cfg(feature = "hardening")))]
pub use crate::hardening::decapsulate_hardened;
