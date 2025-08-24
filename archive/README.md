# Archive - Legacy Implementation Files

This folder contains the original implementation files that have been consolidated into the new `src/math.rs` module for better FIPS 203 compliance and maintainability.

## Archived Files

### `ntt_legacy.rs`

* **Original location**: `src/ntt.rs`
* **Purpose**: Number Theoretic Transform implementation
* **Status**: Consolidated into `src/math.rs` as `ntt_forward()` and `ntt_inverse()`
* **FIPS 203**: Algorithms 9-10

### `poly_legacy.rs`

* **Original location**: `src/poly.rs`
* **Purpose**: Polynomial arithmetic operations
* **Status**: Consolidated into `src/math.rs` as `Poly` struct and methods
* **FIPS 203**: Section 2.1

### `utils_legacy.rs`

* **Original location**: `src/utils.rs`
* **Purpose**: Cryptographic hash functions, sampling, and encoding
* **Status**: Consolidated into `src/math.rs` as hash functions, sampling algorithms, and encoding functions
* **FIPS 203**: Algorithms 4-8, Section 4.1

## Migration Notes

All mathematical operations have been consolidated into `src/math.rs` with:
* Clear FIPS 203 algorithm references
* Improved documentation and comments
* Consistent naming conventions
* Better organization by algorithm type

The new `src/math.rs` module provides a single source of truth for all mathematical operations, making it easier to verify FIPS 203 compliance and maintain the codebase.

## Usage

These files are kept for reference only. All new code should use the functions from `src/math.rs` .
