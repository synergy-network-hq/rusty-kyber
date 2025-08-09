## Core primitives
- [ ] Decomposition helpers: `power2round`, `decompose`, `hint` *(Dilithium-only; parity item)*

---

## Packing / encoding
- [~] Pack/unpack polynomials (4/5/10/11/12/13 bits) — generic d-bit codec in place; audit & tests pending

---

## KATs and testing
- [ ] 100% unit/integration coverage (core + KEM)
- [~] Fuzz tests (packing/decoding, KEM ops)

---

## Portability and builds
- [~] `no_std` readiness (feature-gated) — builds; more tests desirable
- [ ] Embedded targets (ARM64) build/test

---

## Performance
- [ ] Micro-optimizations (NTT, polynomial arithmetic, batching)

---

## Security and auditability
- [ ] Optional masking/hardening (first-order masking, isolation guidance in code)

---

## Docs & distribution
- [~] Crates.io readiness: metadata filled, docs.rs config, CI packaging dry-run; final publish steps in RELEASING.md

---

## Additional tasks & housekeeping
- [ ] Remove unused directories: delete `PQClean/` after migrating vectors
- [ ] Prepare for crates.io publication: metadata, release notes, versioning policy
