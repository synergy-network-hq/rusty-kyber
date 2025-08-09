//! Public API newtypes and ergonomic wrappers for Kyber.
//!
//! This layer provides typed wrappers around the byte-oriented KEM core
//! (key generation, encapsulation, decapsulation), plus optional helpers
//! behind feature gates (`std`, `serde`, `zeroize`).
//!
//! ## Examples
//! Generate a keypair and perform a round-trip:
//! ```no_run
//! use rand::rngs::OsRng;
//! use rusty_kyber::{keypair, encapsulate, decapsulate};
//! let (pk, sk) = keypair(&mut OsRng);
//! let (ct, ss_sender) = encapsulate(&mut OsRng, &pk);
//! let ss_receiver = decapsulate(&sk, &ct);
//! assert_eq!(ss_sender.as_bytes(), ss_receiver.as_bytes());
//! ```
//!
//! With `std` enabled, you can use the `*_osrng` convenience helpers.

#![allow(clippy::needless_pass_by_value)]

use crate::kem;
use crate::params::{
    CIPHERTEXT_BYTES, PUBLIC_KEY_BYTES, SECRET_KEY_BYTES, SHARED_SECRET_BYTES,
};

use core::fmt;

#[cfg(feature = "std")]
use rand::rngs::OsRng;
use rand_core::{CryptoRng, RngCore};

/// Kyber public key (`PUBLIC_KEY_BYTES`).
///
/// Stable byte layout: `t (K*poly) || rho (32)`.
#[derive(Clone, Eq, PartialEq)]
pub struct PublicKey(pub [u8; PUBLIC_KEY_BYTES]);

/// Kyber secret key (`SECRET_KEY_BYTES`).
///
/// Stable byte layout: `s (K*poly) || pk || H(pk) || z`.
/// If the `zeroize` feature is enabled, the buffer is wiped on `drop`.
#[derive(Eq, PartialEq)]
pub struct SecretKey(pub [u8; SECRET_KEY_BYTES]);

/// Kyber ciphertext (`CIPHERTEXT_BYTES`).
///
/// Stable byte layout: `u (K polys, DU bits) || v (1 poly, DV bits)`.
#[derive(Clone, Eq, PartialEq)]
pub struct Ciphertext(pub [u8; CIPHERTEXT_BYTES]);

/// Kyber shared secret (always 32 bytes).
#[derive(Clone, Eq, PartialEq)]
pub struct SharedSecret(pub [u8; SHARED_SECRET_BYTES]);

// ----- serde (feature-gated) -----
// We serialize the newtypes as raw fixed-length byte strings for compactness.
#[cfg(feature = "serde")]
mod serde_impls {
    use super::*;
    use core::marker::PhantomData;
    use serde::{
        de::{Error as DeError, Visitor},
        Deserialize, Deserializer, Serialize, Serializer,
    };

    struct BytesVisitor<const N: usize>(PhantomData<[u8; N]>);

    impl<'de, const N: usize> Visitor<'de> for BytesVisitor<N> {
        type Value = [u8; N];
        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "exactly {} bytes", N)
        }
        fn visit_bytes<E: DeError>(self, v: &[u8]) -> Result<Self::Value, E> {
            if v.len() != N {
                return Err(E::invalid_length(v.len(), &self));
            }
            let mut out = [0u8; N];
            out.copy_from_slice(v);
            Ok(out)
        }
        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let mut out = [0u8; N];
            for i in 0..N {
                out[i] = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(i, &self))?;
            }
            Ok(out)
        }
    }

    impl Serialize for PublicKey {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            s.serialize_bytes(&self.0)
        }
    }
    impl<'de> Deserialize<'de> for PublicKey {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let arr: [u8; PUBLIC_KEY_BYTES] =
                d.deserialize_bytes(BytesVisitor::<PUBLIC_KEY_BYTES>(PhantomData))?;
            Ok(PublicKey(arr))
        }
    }

    impl Serialize for SecretKey {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            s.serialize_bytes(&self.0)
        }
    }
    impl<'de> Deserialize<'de> for SecretKey {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let arr: [u8; SECRET_KEY_BYTES] =
                d.deserialize_bytes(BytesVisitor::<SECRET_KEY_BYTES>(PhantomData))?;
            Ok(SecretKey(arr))
        }
    }

    impl Serialize for Ciphertext {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            s.serialize_bytes(&self.0)
        }
    }
    impl<'de> Deserialize<'de> for Ciphertext {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let arr: [u8; CIPHERTEXT_BYTES] =
                d.deserialize_bytes(BytesVisitor::<CIPHERTEXT_BYTES>(PhantomData))?;
            Ok(Ciphertext(arr))
        }
    }

    impl Serialize for SharedSecret {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            s.serialize_bytes(&self.0)
        }
    }
    impl<'de> Deserialize<'de> for SharedSecret {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let arr: [u8; SHARED_SECRET_BYTES] =
                d.deserialize_bytes(BytesVisitor::<SHARED_SECRET_BYTES>(PhantomData))?;
            Ok(SharedSecret(arr))
        }
    }
}

// ----- zeroize (feature-gated) -----
#[cfg(feature = "zeroize")]
impl Drop for SecretKey {
    fn drop(&mut self) {
        use zeroize::Zeroize;
        self.0.zeroize();
    }
}

impl fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PublicKey(len={})", self.0.len())
    }
}
impl fmt::Debug for Ciphertext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ciphertext(len={})", self.0.len())
    }
}
impl fmt::Debug for SharedSecret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SharedSecret(len={})", self.0.len())
    }
}

// Intentionally omit Debug for SecretKey.

impl PublicKey {
    /// Borrow the underlying bytes.
    #[inline]
    pub fn as_bytes(&self) -> &[u8; PUBLIC_KEY_BYTES] {
        &self.0
    }
    /// Consume and return the underlying bytes.
    #[inline]
    pub fn into_bytes(self) -> [u8; PUBLIC_KEY_BYTES] {
        self.0
    }
    /// Construct from bytes (no validation needed; runtime invariant is size).
    #[inline]
    pub fn from_bytes(bytes: [u8; PUBLIC_KEY_BYTES]) -> Self {
        Self(bytes)
    }
}
impl SecretKey {
    /// Borrow the underlying bytes.
    #[inline]
    pub fn as_bytes(&self) -> &[u8; SECRET_KEY_BYTES] {
        &self.0
    }
    /// Consume and return the underlying bytes.
    #[inline]
    pub fn into_bytes(self) -> [u8; SECRET_KEY_BYTES] {
        self.0
    }
    /// Construct from bytes (no validation needed; runtime invariant is size).
    #[inline]
    pub fn from_bytes(bytes: [u8; SECRET_KEY_BYTES]) -> Self {
        Self(bytes)
    }
}
impl Ciphertext {
    /// Borrow the underlying bytes.
    #[inline]
    pub fn as_bytes(&self) -> &[u8; CIPHERTEXT_BYTES] {
        &self.0
    }
    /// Consume and return the underlying bytes.
    #[inline]
    pub fn into_bytes(self) -> [u8; CIPHERTEXT_BYTES] {
        self.0
    }
    /// Construct from bytes (no validation needed; runtime invariant is size).
    #[inline]
    pub fn from_bytes(bytes: [u8; CIPHERTEXT_BYTES]) -> Self {
        Self(bytes)
    }
}
impl SharedSecret {
    /// Borrow the underlying bytes.
    #[inline]
    pub fn as_bytes(&self) -> &[u8; SHARED_SECRET_BYTES] {
        &self.0
    }
    /// Consume and return the underlying bytes.
    #[inline]
    pub fn into_bytes(self) -> [u8; SHARED_SECRET_BYTES] {
        self.0
    }
    /// Construct from bytes (no validation needed; runtime invariant is size).
    #[inline]
    pub fn from_bytes(bytes: [u8; SHARED_SECRET_BYTES]) -> Self {
        Self(bytes)
    }
}

/// Generate a Kyber keypair using the provided RNG.
///
/// On success returns `(PublicKey, SecretKey)`.
pub fn keypair<R: RngCore + CryptoRng>(rng: &mut R) -> (PublicKey, SecretKey) {
    let mut pk = [0u8; PUBLIC_KEY_BYTES];
    let mut sk = [0u8; SECRET_KEY_BYTES];
    kem::keygen(rng, &mut pk, &mut sk);
    (PublicKey(pk), SecretKey(sk))
}

/// Generate a Kyber keypair using the OS RNG (`std` feature).
#[cfg(feature = "std")]
#[cfg_attr(not(test), allow(dead_code))]
pub fn keypair_osrng() -> (PublicKey, SecretKey) {
    let mut rng = OsRng;
    keypair(&mut rng)
}

/// Encapsulate to `pk` using the provided RNG.
///
/// Returns `(Ciphertext, SharedSecret)`.
pub fn encapsulate<R: RngCore + CryptoRng>(
    rng: &mut R,
    pk: &PublicKey,
) -> (Ciphertext, SharedSecret) {
    let mut ct = [0u8; CIPHERTEXT_BYTES];
    let mut ss = [0u8; SHARED_SECRET_BYTES];
    kem::encaps(rng, &pk.0, &mut ss, &mut ct);
    (Ciphertext(ct), SharedSecret(ss))
}

/// Encapsulate to `pk` using the OS RNG (`std` feature).
#[cfg(feature = "std")]
#[cfg_attr(not(test), allow(dead_code))]
pub fn encapsulate_osrng(pk: &PublicKey) -> (Ciphertext, SharedSecret) {
    let mut rng = OsRng;
    encapsulate(&mut rng, pk)
}

/// Decapsulate `ct` with `sk`, returning the shared secret.
pub fn decapsulate(sk: &SecretKey, ct: &Ciphertext) -> SharedSecret {
    let mut ss = [0u8; SHARED_SECRET_BYTES];
    kem::decaps(&sk.0, &ct.0, &mut ss);
    SharedSecret(ss)
}
