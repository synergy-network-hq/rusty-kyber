# rusty-kyber — File Tree

*(updated: 2025-08-12)*

rusty-kyber/
├── benches/
│   ├── benchmark.rs
│   ├── kem.rs
│   ├── ntt.rs
│   └── poly.rs
├── examples/
│   └── no_std.rs
├── fuzz/
│   ├── Cargo.toml
│   ├── README.md
│   ├── artifacts/
│   │   └── pack_unpack/
│   ├── corpus/
│   │   └── pack_unpack/
│   └── fuzz_targets/
│       ├── kem_roundtrip.rs
│       └── pack_unpack.rs
├── src/
│   ├── api.rs
│   ├── decaps.rs
│   ├── encaps.rs
│   ├── hardening.rs
│   ├── kem.rs
│   ├── keygen.rs
│   ├── lib.rs
│   ├── ntt.rs
│   ├── params.rs
│   ├── poly.rs
│   └── utils.rs
├── tests/
│   ├── api_std.rs
│   ├── arm64_smoke.rs
│   ├── cbd.rs
│   ├── ctr_drbg.rs
│   ├── hardening.rs
│   ├── hardening_ok.rs
│   ├── kat.rs
│   ├── kat_vectors/
│   │   ├── kyber1024_clean.rsp
│   │   ├── kyber1024.rsp
│   │   ├── kyber512_clean.rsp
│   │   ├── kyber512.rsp
│   │   ├── kyber768_clean.rsp
│   │   └── kyber768.rsp
│   ├── kem_fail.rs
│   ├── ntt_mul.rs
│   ├── ntt_poly.rs
│   ├── pack.rs
│   ├── params_consistency.rs
│   ├── serde.rs
│   ├── utils_hash.rs
│   ├── utils_msg.rs
│   └── xof_matrix.rs
├── Cargo.lock
├── Cargo.toml
├── CHANGELOG.md
├── LICENSE-APACHE
├── LICENSE-MIT
├── README.md
├── RELEASING.md
├── ROADMAP.md
├── SECURITY.md
└── to-do.md
