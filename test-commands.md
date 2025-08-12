1) KATs + unit/integration tests (all levels / feature combos)
bash
Copy
Edit
# Default (std + kyber512)
cargo test --all --verbose

# std + kyber768 / kyber1024
cargo test --no-default-features --features "std,kyber768" --verbose
cargo test --no-default-features --features "std,kyber1024" --verbose

# serde + zeroize (API surfaces, drop behavior)
cargo test --no-default-features --features "std,kyber512,serde,zeroize" --verbose

# hardening path (redundant decaps)
cargo test --no-default-features --features "std,kyber512,hardening" --verbose
What to check:

All tests pass, including tests/kat.rs (byte-for-byte pk/sk/ct/ss).

No panics in kem_fail.rs, hardening_ok.rs, hashing, samplers, codecs, NTT equivalence.

2) Coverage (enforced gate)
We added a 90% coverage gate in CI. Locally:

bash
Copy
Edit
cargo install cargo-tarpaulin
cargo tarpaulin --out Lcov --timeout 600 --verbose --fail-under 90
# Optional: view LCOV in a GUI like lcov/genhtml if you want a report.
Goal:

Hit ≥90% (or bump the gate if you want stricter).

3) Fuzzing (quick smoke + longer soaks)
Quick 60s soaks (same as docs):

bash
Copy
Edit
rustup toolchain install nightly
cargo +nightly install cargo-fuzz

cargo +nightly fuzz run pack_unpack -- -max_total_time=60
cargo +nightly fuzz run kem_roundtrip -- -max_total_time=60
Longer, when you’ve got time (recommended before release):

bash
Copy
Edit
cargo +nightly fuzz run pack_unpack -- -max_total_time=1800
cargo +nightly fuzz run kem_roundtrip -- -max_total_time=1800
Expect:

No crashes. If there are, artifacts land in fuzz/artifacts/<target>/ for triage.

4) no_std + wasm + embedded (ARM64) builds
bash
Copy
Edit
# no_std check for kyber512
cargo check --no-default-features --features kyber512

# wasm32 build (no_std)
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --no-default-features --features kyber512

# ARM64 cross build + runtime smoke (requires Docker)
cargo install cross --git https://github.com/cross-rs/cross
cross build --target aarch64-unknown-linux-gnu
cross run   --target aarch64-unknown-linux-gnu --release --example arm64_smoke --features "std,kyber512"
Watch for:

Successful run prints a small “arm64 smoke ok …” line and exits 0.

5) Lints, docs, reproducible build surface
bash
Copy
Edit
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
cargo package --allow-dirty # verify packaging works (uses Cargo.toml exclude list)
Everything should pass with zero warnings (we left a couple non-critical unused_parens in utils.rs; happy to sweep those too if you want absolute silence).

6) Optional side-channel sanity (timing)
If you want a quick constant-time check beyond code review:

Run a dudect-style t-test comparing two input classes (valid vs. invalid CT) against decapsulate. There’s a dudect/dudect-rs approach you can wire as a dev-only harness. Not a formal proof, but it catches gross leaks.

Also ensure your deployment compiles with LTO and panic=abort if you care about minimizing side effects (documented in SECURITY.md).

7) Dependency hygiene & UB checks
bash
Copy
Edit
cargo audit         # CVEs in transitive deps
cargo +nightly miri test  # UB and undefined-behavior checks in tests (safe Rust should be clean)
(We’re #![forbid(unsafe_code)], so Miri is mostly paranoia, but it’s quick.)

8) Release dry-run & publish
Update CHANGELOG.md + bump version in Cargo.toml.

Ensure CARGO_REGISTRY_TOKEN secret is set in your repo (for the release workflow).

Do a final local dry run:

bash
Copy
Edit
cargo publish --dry-run
Tag to trigger the workflow:

bash
Copy
Edit
git tag v0.1.0
git push origin v0.1.0
# release.yml will publish automatically
