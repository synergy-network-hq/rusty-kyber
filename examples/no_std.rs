#![no_std]

//! Minimal no_std smoke example. This doesn't execute anything at runtime;
//! it's built in CI for `wasm32-unknown-unknown` with no default features,
//! which ensures the crate compiles in a pure no_std environment.

#[no_mangle]
pub extern "C" fn kyber_no_std_smoke() {
    // Touch some public constants/types so they are referenced
    let _pk = rusty_kyber::params::PUBLIC_KEY_BYTES;
    let _sk = rusty_kyber::params::SECRET_KEY_BYTES;
    let _ct = rusty_kyber::params::CIPHERTEXT_BYTES;
    let _ss = rusty_kyber::params::SHARED_SECRET_BYTES;

    // Construct newtypes via zero-initialized arrays (no RNG, no std)
    let _pk_t = rusty_kyber::api::PublicKey([0u8; rusty_kyber::params::PUBLIC_KEY_BYTES]);
    let _ct_t = rusty_kyber::api::Ciphertext([0u8; rusty_kyber::params::CIPHERTEXT_BYTES]);

    // No-op: just ensures linking and no_std paths are valid.
}
