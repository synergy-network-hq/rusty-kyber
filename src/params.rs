//! Kyber (ML-KEM) parameter selection and derived sizes.
//!
//! # Feature flags
//! - **Levels** (select one): `kyber512` *(default)*, `kyber768`, `kyber1024`
//! - If multiple are enabled (e.g. via `--features "kyber768"` without
//!   `--no-default-features`), we deterministically select the first in
//!   priority order **512 → 768 → 1024** to avoid duplicate definitions.
//!
//! This module exposes compile-time constants used across the implementation.
//! All values are public and stable for a given feature set.

/// Ring dimension (*n* = 256).
pub const N: usize = 256;

/// Prime modulus (*q* = 3329).
pub const Q: i32 = 3329;

/// Bytes of a 256-bit string used by Kyber (`rho`, `sigma`, `z`, hashes).
pub const SYMBYTES: usize = 32;

/// Number of bytes used to serialize a polynomial (256 coeffs × 12 bits).
pub const POLY_BYTES: usize = 384;

// ----- Level selection (compile-time) -----
// Priority: 512 → 768 → 1024

/// Whether `kyber512` is the active level.
pub const KYBER_512: bool = cfg!(feature = "kyber512");

/// Whether `kyber768` is the active level (only if 512 is not active).
pub const KYBER_768: bool = cfg!(feature = "kyber768") && !KYBER_512;

/// Whether `kyber1024` is the active level (only if 512/768 are not active).
pub const KYBER_1024: bool = cfg!(feature = "kyber1024") && !KYBER_512 && !KYBER_768;

/// Module rank *k* (number of polynomials per vector).
pub const K: usize = if KYBER_512 {
    2
} else if KYBER_768 {
    3
} else if KYBER_1024 {
    4
} else {
    // Fallback to 512 if none explicitly selected.
    2
};

/// Centered binomial parameter η₁.
/// - 3 for Kyber-512
/// - 2 for Kyber-768/1024
pub const ETA1: i32 = if KYBER_512 { 3 } else { 2 };

/// Centered binomial parameter η₂ (always 2).
pub const ETA2: i32 = 2;

/// Compression bits for `u` in the ciphertext.
/// - 10 for Kyber-512/768
/// - 11 for Kyber-1024
pub const DU: usize = if KYBER_1024 { 11 } else { 10 };

/// Compression bits for `v` in the ciphertext.
/// - 4 for Kyber-512/768
/// - 5 for Kyber-1024
pub const DV: usize = if KYBER_1024 { 5 } else { 4 };

// ----- Derived sizes -----

/// Size of a polynomial vector: `K * POLY_BYTES`.
pub const POLYVEC_BYTES: usize = K * POLY_BYTES;

/// Public key size: `t (K*poly) || rho (32)`.
pub const PUBLIC_KEY_BYTES: usize = POLYVEC_BYTES + SYMBYTES;

/// Secret key size: `s (K*poly) || pk || H(pk) || z`.
pub const SECRET_KEY_BYTES: usize = POLYVEC_BYTES + PUBLIC_KEY_BYTES + SYMBYTES + SYMBYTES;

/// Ciphertext size: `u (K polys, DU bits each) || v (1 poly, DV bits)`.
pub const CIPHERTEXT_BYTES: usize = K * (DU * N / 8) + (DV * N / 8);

/// Shared secret size (32 bytes).
pub const SHARED_SECRET_BYTES: usize = SYMBYTES;
