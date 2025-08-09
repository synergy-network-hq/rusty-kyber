#![no_std]
#![no_main]

// Minimal no_std smoke. Built for wasm32-unknown-unknown in CI.
// Provide an entry point so the example links cleanly.

#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn _start() {
    // Touch public constants to ensure paths resolve.
    let _ = rusty_kyber::params::PUBLIC_KEY_BYTES;
    let _ = rusty_kyber::params::SECRET_KEY_BYTES;
    let _ = rusty_kyber::params::CIPHERTEXT_BYTES;
    let _ = rusty_kyber::params::SHARED_SECRET_BYTES;
}

#[cfg(not(target_arch = "wasm32"))]
#[no_mangle]
pub extern "C" fn main() {}
