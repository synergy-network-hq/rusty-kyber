# rusty-kyber — File Tree
*(updated: 2025-08-08)*

rusty-kyber/
├── fuzz/
│   ├── Cargo.toml
│   └── fuzz_targets/
│      ├── kem_roundtrip.rs
│      └── pack_unpack.rs
├── examples/
│   └── no_std.rs # NEW: no_std smoke (wasm32)
├── .github/
│   └── workflows/
│   └── ci.yml
├── .gitignore
├── Cargo.toml
├── CHANGELOG.md
├── LICENSE-APACHE
├── LICENSE-MIT
├── README.md
├── RELEASING.md
├── SECURITY.md
├── to-do.md
├── file-tree.md
├── benches/
│   ├── kem.rs
│   ├── ntt.rs
│   └── poly.rs
├── src/
│   ├── lib.rs
│   ├── params.rs
│   ├── ntt.rs
│   ├── poly.rs
│   ├── utils.rs
│   ├── keygen.rs
│   ├── encaps.rs
│   ├── decaps.rs
│   ├── kem.rs
│   └── api.rs
├── tests/
│   ├── ntt_poly.rs # NEW
│   ├── utils_msg.rs # NEW
│   ├── pack.rs
│   ├── ctr_drbg.rs
│   ├── kat.rs
│   └── kat_vectors/
│       ├── kyber512.rsp
│       ├── kyber512_clean.rsp
│       ├── kyber768.rsp
│       ├── kyber768_clean.rsp
│       ├── kyber1024.rsp
│       └── kyber1024_clean.rsp
└── PQClean/
