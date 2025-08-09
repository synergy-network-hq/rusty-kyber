//! Polynomial type and basic operations over R_q = Z_q[X]/(X^256 + 1) with q = 3329.
//!
//! This module provides a compact `Poly` representation and in-place primitives
//! used throughout Kyber: addition, subtraction, NTT/InvNTT, and pointwise
//! multiplication in the NTT domain. Reduction is integrated where needed.

use crate::ntt;
use crate::params::{N, Q};

/// A degree-255 polynomial with coefficients modulo `Q`.
#[derive(Clone, Copy)]
pub struct Poly {
    /// Coefficients in canonical order; the current domain (normal vs NTT) is
    /// tracked by callers and operations invoked (`ntt()`, `inv_ntt()`).
    pub coeffs: [i16; N],
}

impl Default for Poly {
    fn default() -> Self {
        Self::new()
    }
}

impl Poly {
    /// Construct the zero polynomial.
    #[inline(always)]
    pub const fn new() -> Self {
        Self { coeffs: [0; N] }
    }

    /// In-place coefficient-wise addition: `self += other` (mod `Q` with wrapping).
    #[inline(always)]
    pub fn add(&mut self, other: &Poly) {
        let a = &mut self.coeffs;
        let b = &other.coeffs;
        for i in 0..N {
            a[i] = a[i].wrapping_add(b[i]);
        }
    }

    /// In-place coefficient-wise subtraction: `self -= other` (mod `Q` with wrapping).
    #[inline(always)]
    pub fn sub(&mut self, other: &Poly) {
        let a = &mut self.coeffs;
        let b = &other.coeffs;
        for i in 0..N {
            a[i] = a[i].wrapping_sub(b[i]);
        }
    }

    /// Forward Number-Theoretic Transform (NTT), in-place.
    #[inline(always)]
    pub fn ntt(&mut self) {
        ntt::ntt(&mut self.coeffs);
    }

    /// Inverse Number-Theoretic Transform (NTT), in-place.
    #[inline(always)]
    pub fn inv_ntt(&mut self) {
        ntt::inv_ntt(&mut self.coeffs);
    }

    /// Pointwise multiplication in the NTT domain, in-place.
    #[inline(always)]
    pub fn pointwise_mul(&mut self, other: &Poly) {
        let a = &mut self.coeffs;
        let b = &other.coeffs;
        for i in 0..N {
            a[i] = montgomery_reduce((a[i] as i32) * (b[i] as i32));
        }
    }
}

/// Batch forward NTT on a slice of polynomials (in place).
///
/// This is a convenience for callers/benches to reduce loop overhead and let
/// the optimizer see a tight hot path; it preserves constant-time behavior.
#[inline(always)]
pub fn ntt_batch(polys: &mut [Poly]) {
    for p in polys {
        ntt::ntt(&mut p.coeffs);
    }
}

/// Batch inverse NTT on a slice of polynomials (in place).
#[inline(always)]
pub fn inv_ntt_batch(polys: &mut [Poly]) {
    for p in polys {
        ntt::inv_ntt(&mut p.coeffs);
    }
}

#[inline(always)]
fn montgomery_reduce(a: i32) -> i16 {
    // R = 2^16, qinv = -q^{-1} mod 2^16 = 62209
    let u = (a as i64 * 62209i64) as i32 & 0xFFFF;
    let t = (a - u * Q) >> 16;
    t as i16
}
