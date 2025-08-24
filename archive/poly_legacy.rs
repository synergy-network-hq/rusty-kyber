//! Polynomial type and basic operations over R_q = Z_q[X]/(X^256 + 1) with q = 3329.
//!
//! This module provides a compact `Poly` representation and in-place primitives
//! used throughout Kyber: addition, subtraction, NTT/InvNTT, and pointwise
//! multiplication in the NTT domain. Reduction is integrated where needed.

use crate::ntt;
use crate::ntt::GAMMAS;
use crate::params::{ N, Q };

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
        let a = &self.coeffs;
        let b = &other.coeffs;
        let mut out = [0i16; N];
        for i in 0..N / 2 {
            let a0 = a[2 * i] as i32;
            let a1 = a[2 * i + 1] as i32;
            let b0 = b[2 * i] as i32;
            let b1 = b[2 * i + 1] as i32;
            let z = GAMMAS[i] as i32;

            let mut c0 = (a0 * b0 + z * ((a1 * b1) % Q)) % Q;
            if c0 < 0 {
                c0 += Q;
            }
            let mut c1 = (a0 * b1 + a1 * b0) % Q;
            if c1 < 0 {
                c1 += Q;
            }
            out[2 * i] = c0 as i16;
            out[2 * i + 1] = c1 as i16;
        }
        self.coeffs = out;
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

// Removed Montgomery reduction to keep arithmetic consistent with field-based NTT
