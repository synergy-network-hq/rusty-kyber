# Changelog

All notable changes to this project will be documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-08-08

### Added

* Pure-Rust ML-KEM (Kyber) implementation for kyber512/768/1024.
* Constant-time decapsulation path; optional `hardening` feature with redundant checks.
* `no_std` support (wasm32 build), fuzz targets, KAT harness for all levels.
* Benches for NTT/poly/KEM; extensive unit tests to near-100% coverage.
* CI: multi-toolchain matrix, wasm, cross aarch64 build + runtime smoke, fuzz smoke runs.
* Docs.rs configuration with feature badges; README badges.

### Changed

* Performance: inlining and batched NTT; branchless canonicalization in codecs.

### Security

* Documented threat model; guidance for isolation and future masking work.

---

## [Unreleased]

### Added

* Streaming SampleNTT helper for uniform polynomials via SHAKE128.
* Deterministic KAT mode in tests using `rho||sigma = G(d||K)` and NTT-domain key storage.
* FO transform aligned with FIPS 203 final: SS = K (encaps); decaps selects between `K'` and `Kbar = J(z||ct)`.

### Changed

* XOF matrix indexing corrected to absorb `(rho || i || j)`.
* `poly_to_msg` updated to use proper rounding for thresholding.
* Public key/secret key packing now stores `t_hat` and `s_hat` in NTT domain.

### Removed

* `ml-kem` dev-dependency (kept as local reference only; folder slated for removal before release).

### Security

* Strengthened constant-time paths; consistent domain handling for KEM operations.
