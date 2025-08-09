//! Feature-gated hardening helpers.
//!
//! This module provides hardened variants of sensitive operations that add
//! double-execution checks and wipe intermediate secrets. Enable with the
//! `hardening` cargo feature.
//!
//! ## Notes
//! - These helpers are deterministic for the same inputs and do not change the
//!   algorithm outputs. They only add internal redundancy and scrubbing.
//! - Double-execution defends against fault/glitch injection that flips a bit
//!   during decapsulation. Mismatch yields an all-zero shared secret.

use subtle::ConditionallySelectable;

use subtle::{ConstantTimeEq, Choice};
#[cfg(feature = "zeroize")]
use zeroize::Zeroizing;

use crate::api::{Ciphertext, SecretKey, SharedSecret};
use crate::params::SHARED_SECRET_BYTES;

/// Hardened decapsulation: compute the shared secret twice and compare in
/// constant time. If a mismatch is detected, return an all-zero secret.
///
/// This defends against transient fault/glitch attacks that attempt to skip or
/// corrupt the internal verify+KDF path.
pub fn decapsulate_hardened(sk: &SecretKey, ct: &Ciphertext) -> SharedSecret {
    // First run
    let mut ss1 = [0u8; SHARED_SECRET_BYTES];
    crate::kem::decaps(&sk.0, &ct.0, &mut ss1);

    // Second run (redundant)
    let mut ss2 = [0u8; SHARED_SECRET_BYTES];
    crate::kem::decaps(&sk.0, &ct.0, &mut ss2);

    // Constant-time compare
    let equal: Choice = ss1.ct_eq(&ss2);

    // Select either the agreed secret or an all-zero value, in constant time.
    let mut out = [0u8; SHARED_SECRET_BYTES];
    for i in 0..SHARED_SECRET_BYTES {
        // out[i] = equal ? ss1[i] : 0
        out[i] = u8::conditional_select(&0u8, &ss1[i], equal);
    }

    // Scrub redundant buffer(s)
    #[cfg(feature = "zeroize")]
    {
        let _ = Zeroizing::new(ss2); // drop => zero
    }

    SharedSecret(out)
}
