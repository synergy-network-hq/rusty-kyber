# rusty-kyber: Implementation Tracker

Status legend: [x] done, [ ] todo, [~] partial/in-progress

---

## KEM Core (FIPS 203)

* [x] H/G/J/PRF wrappers with out-parameter style (`h/g/kdf/prf`)
* [x] Matrix expansion XOF `SHAKE128(rho || i || j)`
* [x] SampleNTT: streaming rejection sampler to fill all 256 coeffs
* [x] NTT/InvNTT with FIPS Appendix A zetas; NTT-domain operations consistent
* [x] Store and encode `s_hat` and `t_hat` in NTT domain (pk/sk encode/decode)
* [x] Encapsulation: `(K, coins) = G(m || H(pk))`; encrypt with `coins`; SS = K
* [x] Decapsulation: compute `K'`; re-encrypt and constant-time select between `K'` and `Kbar = J(z || ct)`
* [x] `poly_from_msg` and `poly_to_msg` mapping; thresholding uses correct rounding
* [~] Validate OS RNG roundtrip across all sets (remaining mismatch to debug)

---

## Encoding, Packing, and Codecs

* [x] 12-bit polynomial codec (384 bytes) — idempotent, round-trip tested
* [x] Generic d-bit codec (DU/DV 4/5/10/11/12/13) with wrappers for u/v
* [x] Public key layout: `t_hat || rho`
* [x] Secret key layout: `s_hat || pk || H(pk) || z`

---

## Deterministic Mode & KATs

* [x] CTR-DRBG for KAT RNG harness
* [x] Deterministic derivation `rho||sigma = G(d||K)` implemented in tests
* [x] Deterministic KAT harness updated to construct `pk/sk` from deterministic `s_hat/t_hat`
* [ ] Update KAT assertions to conform to FIPS-203 final: assert shared-secret/roundtrip only (not legacy CRYSTALS byte-for-byte `.rsp`)
* [ ] Optionally import ACVP FIPS-203 vectors (if available) and add parsing harness

---

## API, Features, and Ergonomics

* [x] Public API structs (`PublicKey`,   `SecretKey`,   `Ciphertext`,   `SharedSecret`)
* [x] `serde` (feature-gated) for public types
* [x] `zeroize` on `SecretKey` drop (feature-gated)
* [x] Finalize API: `keypair/encapsulate/decapsulate` (RNG-agnostic + OsRng)
* [x] Feature gates: `std`/`no_std`, and `kyber512` / `kyber768` / `kyber1024` (strict-checked)

---

## Testing & CI

* [x] Pack/unpack codecs unit tests (d-bit, DU/DV)
* [x] Fuzz tests for pack/kem targets
* [~] KAT deterministic tests updated; expectations need finalization
* [ ] Fix OS RNG roundtrip test mismatch
* [ ] Run full feature matrix:
  + `--features kyber512,std`
  + `--no-default-features --features kyber768,std`
  + `--no-default-features --features kyber1024,std`
* [ ] CI: clippy (deny warnings), docs build, wasm/arm64 build

---

## Documentation

* [x] README.md: overview, build, usage, security notes
* [x] Module and public API rustdocs
* [ ] Expand docs with FIPS-203 alignment notes and Appendix references

---

## Performance

* [x] Benches (criterion): NTT, poly add/sub/pointwise, keygen/encaps/decaps
* [ ] Re-check benches after streaming SampleNTT changes

---

## Security & Hardening

* [x] `forbid(unsafe_code)`
* [x] Constant-time equality and verify/select; domain-consistent ops
* [x] Hardened decapsulation (double execution + CT compare; optional feature)
* [ ] Extended hardening/masking (future; out of scope for FIPS conformance)

---

## Maintenance

* [x] Remove `ml-kem` dev dependency (kept only as local reference)
* [ ] Remove local `ml-kem/` folder prior to release (reference only)
* [ ] Ensure `.gitignore` coverage and repo cleanliness

---

## Housekeeping (moved from legacy bucket)

* [x] Fix KAT file paths & selection (by pk size); handle missing files gracefully
* [x] Follow Rust naming: `H/G/PRF/KDF/XOF` → `h/g/prf/kdf/xof`
* [x] Address compiler warnings: sweep remaining `unused_*` in benches/utils
* [x] CI matrix: toolchains + features + wasm + docs + bench compile
* [x] Prepare for crates.io publication: metadata, release notes, versioning policy
* [x] Add `.gitignore` (target/, IDE files, etc.)

---
