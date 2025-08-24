//! ML-KEM Parameter Sets and Constants (FIPS 203 Table 2)
//!
//! This module defines the parameter sets for ML-KEM-512, ML-KEM-768, and ML-KEM-1024
//! exactly as specified in NIST FIPS 203, Table 2. All constants are compile-time
//! determined based on feature flags.
//!
//! ## FIPS 203 Parameter Selection
//! - **ML-KEM-512**: `kyber512` *(default)* - k=2, η₁=3, η₂=2, dᵤ=10, dᵥ=4
//! - **ML-KEM-768**: `kyber768` - k=3, η₁=2, η₂=2, dᵤ=10, dᵥ=4
//! - **ML-KEM-1024**: `kyber1024` - k=4, η₁=2, η₂=2, dᵤ=11, dᵥ=5
//!
//! Priority order: 512 → 768 → 1024 (if multiple features enabled)

/// Ring dimension n = 256 (FIPS 203 Section 2.1)
/// All ML-KEM operations work in the ring R_q = Z_q[X]/(X^256 + 1)
pub const N: usize = 256;

/// Prime modulus q = 3329 (FIPS 203 Section 2.1)
/// All arithmetic is performed modulo this prime
pub const Q: i32 = 3329;

/// Symbol bytes = 32 (FIPS 203 Section 4.1)
/// Length of hash outputs, seeds, and random values
pub const SYMBYTES: usize = 32;

/// Polynomial encoding bytes = 384 (FIPS 203 Section 2.3)
/// Each polynomial uses 256 coefficients × 12 bits = 3072 bits = 384 bytes
pub const POLY_BYTES: usize = 384;

// =============================================================================
// ML-KEM Parameter Set Selection (FIPS 203 Table 2)
// =============================================================================

/// ML-KEM-512 parameter set active
pub const KYBER_512: bool = cfg!(feature = "kyber512");

/// ML-KEM-768 parameter set active (only if ML-KEM-512 not active)
pub const KYBER_768: bool = cfg!(feature = "kyber768") && !KYBER_512;

/// ML-KEM-1024 parameter set active (only if ML-KEM-512/768 not active)
pub const KYBER_1024: bool = cfg!(feature = "kyber1024") && !KYBER_512 && !KYBER_768;

/// Module rank k (FIPS 203 Table 2)
/// - ML-KEM-512: k = 2
/// - ML-KEM-768: k = 3
/// - ML-KEM-1024: k = 4
pub const K: usize = if KYBER_512 {
    2
} else if KYBER_768 {
    3
} else if KYBER_1024 {
    4
} else {
    // Default to ML-KEM-512 if no feature selected
    2
};

/// Centered binomial parameter η₁ (FIPS 203 Table 2)
/// - ML-KEM-512: η₁ = 3
/// - ML-KEM-768: η₁ = 2
/// - ML-KEM-1024: η₁ = 2
pub const ETA1: i32 = if KYBER_512 { 3 } else { 2 };

/// Centered binomial parameter η₂ (FIPS 203 Table 2)
/// - All parameter sets: η₂ = 2
pub const ETA2: i32 = 2;

/// Ciphertext compression parameter dᵤ (FIPS 203 Table 2)
/// - ML-KEM-512: dᵤ = 10
/// - ML-KEM-768: dᵤ = 10
/// - ML-KEM-1024: dᵤ = 11
pub const DU: usize = if KYBER_1024 { 11 } else { 10 };

/// Ciphertext compression parameter dᵥ (FIPS 203 Table 2)
/// - ML-KEM-512: dᵥ = 4
/// - ML-KEM-768: dᵥ = 4
/// - ML-KEM-1024: dᵥ = 5
pub const DV: usize = if KYBER_1024 { 5 } else { 4 };

// =============================================================================
// Derived Key and Ciphertext Sizes (FIPS 203 Table 3)
// =============================================================================

/// Polynomial vector encoding size = k × 384 bytes
pub const POLYVEC_BYTES: usize = K * POLY_BYTES;

/// ML-KEM public key size (FIPS 203 Section 5.2)
/// Layout: t̂ (k×384 bytes) || ρ (32 bytes)
pub const PUBLIC_KEY_BYTES: usize = POLYVEC_BYTES + SYMBYTES;

/// ML-KEM secret key size (FIPS 203 Section 5.2)
/// Layout: ŝ (k×384 bytes) || pk (PUBLIC_KEY_BYTES) || H(pk) (32 bytes) || z (32 bytes)
pub const SECRET_KEY_BYTES: usize = POLYVEC_BYTES + PUBLIC_KEY_BYTES + SYMBYTES + SYMBYTES;

/// ML-KEM ciphertext size (FIPS 203 Section 5.2)
/// Layout: u (k×dᵤ×32 bytes) || v (dᵥ×32 bytes)
pub const CIPHERTEXT_BYTES: usize = K * ((DU * N) / 8) + (DV * N) / 8;

/// ML-KEM shared secret size = 32 bytes (FIPS 203 Section 5.2)
pub const SHARED_SECRET_BYTES: usize = SYMBYTES;
