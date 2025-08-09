# Security Notes

This document summarizes the threat model, constant-time guarantees, RNG usage,
key handling, and side-channel considerations for `rusty-kyber`.

## Scope & goals

- Pure-Rust ML-KEM (Kyber) with constant-time core operations.
- `#![forbid(unsafe_code)]` across the crate.
- Reproducible and byte-for-byte KATs (NIST .rsp) for kyber512/768/1024.
- Optional ergonomics: `serde` for public types, `zeroize` for key erasure.

## Threat model

**In scope**
- Local timing differences at the function boundary (microbench granularity).
- Secret-dependent branches and memory lookups inside primitives.
- Observable differences between failure/success paths in decapsulation.
- Accidental leakage via `Debug`/serialization of secret types.

**Out of scope**
- Microarchitectural attacks that require detailed platform modeling (e.g.,
  cache line contention in co-tenancy, TLB/page-fault channels, branch target
  injection, Spectre-class gadgets on the host).
- Fault injection / glitching.
- Power/EM side-channels (no masking is implemented).

If you need hardening against strong local attackers (co-tenants, shared
hyperthreads, or physical attackers), see **Hardening guidance** below.

## Constant-time policy

We aim for _data-independent control flow_ and _data-independent memory access_
for all operations involving secrets:

- **Decapsulation**: re-encrypt check is done in constant time using
  `subtle::Choice` for equality and `u8::conditional_select` to select between
  `k` and `z`. There are no early returns based on secret data.
- **NTT / poly arithmetic**: fixed iteration counts, no secret-dependent branches.
- **Packing/encoding**: branches depend only on public sizes and loop indices.
- **RNG**: randomness is not derived from secrets; failures (e.g., I/O) do not
  expose secret material.

### Known public vs. secret data

- **Public**: parameter set, lengths, algorithm level (`kyber{512,768,1024}`),
  public key bytes, ciphertext bytes.
- **Secret**: secret key bytes, sampled noise polynomials, encapsulation coins,
  shared secret, intermediate decrypted message.

### Audited constant-time hotspots

- `kem::decaps`: constant-time verify & KDF selection (`subtle`).
- `ntt::{ntt, inv_ntt}`: loop bounds and memory access are index-only.
- `utils::{poly_compress_d, poly_decompress_d}`: no secret-dependent branches;
  scaling and reductions use arithmetic only.
- `utils::{h,g,kdf,prf,xof_matrix}`: SHA3/SHAKE APIs are constant-time with
  respect to input values (control flow depends on input length only).

## Key handling

- `SecretKey` implements zeroization on drop when the **`zeroize`** feature is
  enabled. Enable this in production builds:
    ```bash
    cargo build --release --features zeroize
    ```

- `Debug` is deliberately **not** implemented for `SecretKey`. Other types avoid
printing secret material.

## RNG usage

- **Production**: use `OsRng` (enabled via the default `std` feature) or supply
an RNG implementing `rand_core::{RngCore,CryptoRng}`.
- **KATs**: the test harness uses a deterministic SP 800-90A CTR-DRBG to match
NIST vectors; this is for tests only.

## Side-channel considerations

- **Branching**: avoided on secrets throughout the implementation.
- **Memory access**: tables (zetas) are fixed; no secret-dependent indexing.
- **Masking**: first-order masking is **not implemented**. If you require EM /
power side-channel resistance, deploy in an environment that offers physical
protections or hardware isolation, or integrate an external masked NTT and
masked samplers (non-trivial; out of scope for this crate presently).
- **Co-tenancy**: on shared infrastructure, prefer pinning to isolated cores,
disable SMT/HT when feasible, and avoid colocating secret operations with
untrusted workloads.

## Build guidance

- Use **release** builds for stable timing:
    ```bash
    RUSTFLAGS="-C lto=thin -C codegen-units=1" cargo build --release
    ```
- For `no_std` targets (e.g., `wasm32-unknown-unknown`, embedded), ensure a
high-quality system RNG is available or inject one explicitly.

## Testing & fuzzing

- KATs: `cargo test` runs byte-for-byte comparisons for all levels.
- Fuzz targets (compile-checked in CI):
- `pack_unpack`: round-trip + idempotence for DU/DV codecs.
- `kem_roundtrip`: end-to-end encaps/decaps equality; corrupted CT must not
  panic.

  ## Hardening (optional)

Enable the `hardening` feature for redundant checks and buffer scrubbing:

- `decapsulate_hardened(sk, ct)`: runs decapsulation twice and compares the two
  shared secrets in constant time (`subtle`). On mismatch (indicative of a
  transient fault/glitch), it returns an all-zero secret. Temporary buffers are
  wiped when the `zeroize` feature is also enabled (pulled in automatically by
  `hardening`).
      ```bash
      cargo build --release --features "hardening"
      ```

Trade-offs: ~2× decapsulation time when using `decapsulate_hardened`. Use this
for high-assurance contexts; the default `decapsulate` remains constant-time
and is sufficient for most applications.


## Reporting vulnerabilities

Please open a private advisory or email the maintainers; if unavailable, use
GitHub security advisories for the repository. We’ll coordinate a fix and a
timely release, then credit reporters in the CHANGELOG (opt-in).
