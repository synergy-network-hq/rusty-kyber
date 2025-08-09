# rusty-kyber: Implementation Tracker

Status legend: [x] done, [ ] todo, [~] partial/in-progress

---

## Core primitives
- [x] Keccak/SHAKE XOF (via `sha3` crate; external dep)
- [x] Uniform/centered binomial samplers (η = 2/3)
- [x] Polynomial ring type with add/sub/reduce
- [x] NTT and InvNTT (native schedule; constant-time)
- [x] NTT pointwise multiply
- [x] Montgomery reduction (mod q)
- [x] Barrett reduction and modular arithmetic
- [x] Compression/decompression of polynomials (per Kyber compression factors)
- [x] Matrix expansion A from `rho` (per security level)

---

## Packing / encoding
- [x] Pack/unpack polynomials (4/5/10/11/12/13 bits) — generic d-bit codec in place; audit & tests pending
- [x] Pack/unpack public key (`seedA || polyvec`)
- [x] Pack/unpack secret key (polyvec)
- [x] Pack/unpack ciphertext
- [x] Shared secret encode/decode (FO transform + KDF)

---

## KEM operations
- [x] RNG integration (`rand_core`/`OsRng`)
- [x] KeyGen: sample `s, e`, expand matrix, encode keys
- [x] Encaps: random coins, encrypt, encode `ct`/`ss`
- [x] Decaps: decrypt, validate, return `ss` (constant-time path)

---

## API and ergonomics
- [x] Public API structs (`PublicKey`, `SecretKey`, `Ciphertext`, `SharedSecret`)
- [x] `serde` (feature-gated) for public types
- [x] `zeroize` on `SecretKey` drop (feature-gated)
- [x] Finalize API: `keypair/encapsulate/decapsulate` (RNG-agnostic + OsRng)
- [x] Feature gates: `std`/`no_std`, and `kyber512` / `kyber768` / `kyber1024` (strict-checked)

---

## KATs and testing
- [x] KAT parser (NIST `.rsp`, byte-for-byte)
- [x] Deterministic CTR-DRBG (SP 800-90A) to reproduce KAT RNG
- [x] End-to-end KATs (all three levels) — full `pk/sk/ct/ss` checks
- [~] 100% unit/integration coverage (core + KEM)
- [x] Fuzz tests (packing/decoding, KEM ops)

---

## Portability and builds
- [x] `no_std` readiness (feature-gated) — builds; more tests desirable
- [x] `wasm32` build (CI target); tests TBD
- [x] Embedded targets (ARM64) build/test
- [x] Reproducible builds (fmt/clippy + docs warnings gated in CI)

---

## Performance
- [x] Benches (criterion): NTT, poly add/sub/pointwise, keygen/encaps/decaps
- [x] Micro-optimizations (NTT, polynomial arithmetic, batching)

---

## Security and auditability
- [x] `forbid(unsafe_code)` baseline
- [x] Constant-time audit notes documented (`SECURITY.md`)
- [~] Optional masking/hardening
  - [x] Hardened decapsulation (double-exec + CT verify; zeroize temporaries)
  - [ ] First-order masking (NTT/samplers) — future work; non-trivial and out-of-scope for this pass


---

## Docs & distribution
- [x] README.md: overview, build, usage, security notes
- [x] Rustdoc for every public API/type (+ top-level module docs) — pass started
- [x] Licenses (MIT/Apache-2.0)
- [x] Rustdoc pass for internal modules (`poly`, `utils`, `ntt`, etc.)
- [x] Crates.io readiness: metadata filled, docs.rs config, CI packaging dry-run; final publish steps in RELEASING.md

---

## Additional tasks & housekeeping
- [x] Fix KAT file paths & selection (by pk size); handle missing files gracefully
- [x] Follow Rust naming: `H/G/PRF/KDF/XOF` → `h/g/prf/kdf/xof`
- [x] Address compiler warnings: sweep remaining `unused_*` in benches/utils
- [x] CI matrix: toolchains + features + wasm + docs + bench compile
- [x] Remove unused directories: delete `PQClean/` after migrating vectors
- [x] Prepare for crates.io publication: metadata, release notes, versioning policy
- [x] Add `.gitignore` (target/, IDE files, etc.)

---
