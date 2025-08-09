# Changelog

All notable changes to this project will be documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-08-08

### Added
- Pure-Rust ML-KEM (Kyber) implementation for kyber512/768/1024.
- Constant-time decapsulation path; optional `hardening` feature with redundant checks.
- `no_std` support (wasm32 build), fuzz targets, KAT harness for all levels.
- Benches for NTT/poly/KEM; extensive unit tests to near-100% coverage.
- CI: multi-toolchain matrix, wasm, cross aarch64 build + runtime smoke, fuzz smoke runs.
- Docs.rs configuration with feature badges; README badges.

### Changed
- Performance: inlining and batched NTT; branchless canonicalization in codecs.

### Security
- Documented threat model; guidance for isolation and future masking work.

---

## [Unreleased]

### Added
- Pure-Rust Kyber (ML-KEM) with feature-gated levels: `kyber512` (default), `kyber768`, `kyber1024`.
- Constant-time KEM decapsulation path using `subtle` (ct equality and conditional select).
- NTT/InvNTT with standard zetas and Montgomery reduction.
- Packing/encoding and polynomial compression helpers (generic d-bit codec).
- Full NIST KAT harness with deterministic CTR-DRBG; vectors under `tests/kat_vectors/`.
- Criterion benches: NTT, poly add/sub/pointwise, keygen/encaps/decaps.
- `serde` and `zeroize` feature gates for public API types.
- CI: toolchain/features matrix, wasm build, docs warnings, bench compile, packaging dry-run.
- Licensing: MIT OR Apache-2.0.

### Changed
- Public API exported at crate root for ergonomic use.

### Security
- `#![forbid(unsafe_code)]` at crate root.
