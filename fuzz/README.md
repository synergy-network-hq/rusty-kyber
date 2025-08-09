# Fuzzing rusty-kyber

This directory is a `cargo-fuzz` workspace with two targets:

- `pack_unpack` — exercises polynomial pack/unpack and DU/DV codecs
- `kem_roundtrip` — end-to-end keygen/encaps/decaps (with corrupted-CT sanity)

## Quickstart

```bash
rustup toolchain install nightly
cargo +nightly install cargo-fuzz

# Build corpus and run for a minute
cargo +nightly fuzz run pack_unpack -- -max_total_time=60
cargo +nightly fuzz run kem_roundtrip -- -max_total_time=60
```

Corpora live under `fuzz/corpus/<target>/`. Add more seed files as needed.
Crashes (if any) go to `fuzz/artifacts/<target>/` for triage.
