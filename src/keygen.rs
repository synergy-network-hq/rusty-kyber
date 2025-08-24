//! ML-KEM Key Generation (FIPS 203 Algorithm 15)
//!
//! This module implements ML-KEM.KeyGen exactly as specified in FIPS 203 Algorithm 15.
//! The algorithm generates public/secret key pairs for the ML-KEM cryptosystem.

use crate::params::{ K, ETA1, POLY_BYTES, PUBLIC_KEY_BYTES };
use crate::math::{ Poly, hash_g, prf, sample_cbd, poly_encode, sample_ntt };
use rand_core::{ CryptoRng, RngCore };

/// ML-KEM IND-CPA Key Generation (FIPS 203 Algorithm 12)
///
/// Generates an IND-CPA public/secret key pair. This is called by the main
/// ML-KEM.KeyGen algorithm (Algorithm 15) which adds additional secret key material.
///
/// ## Algorithm Steps (FIPS 203 Algorithm 12)
/// 1. Generate random seed d and derive (ρ, σ) ← G(d || k)
/// 2. Sample secret vectors s, e from CBD_η₁(PRF(σ, N))
/// 3. Compute Â from ρ using SampleNTT
/// 4. Compute t̂ ← Â ◦ ŝ + ê (in NTT domain)
/// 5. Return pk = (t̂, ρ) and sk = ŝ
pub fn indcpa_keypair<R: RngCore + CryptoRng>(
    rng: &mut R,
    pk: &mut [u8; PUBLIC_KEY_BYTES],
    sk: &mut [u8; K * POLY_BYTES]
) {
    // Step 1: Generate seed d and derive ρ || σ ← G(d || k) (FIPS 203 Algorithm 12, line 1-2)
    let mut seed = [0u8; 32];
    rng.fill_bytes(&mut seed);
    let mut input = [0u8; 33];
    input[..32].copy_from_slice(&seed);
    input[32] = K as u8; // Domain separation by parameter set
    let mut buf64 = [0u8; 64];
    hash_g(&input, &mut buf64);
    let rho: [u8; 32] = buf64[..32].try_into().unwrap();
    let sigma: [u8; 32] = buf64[32..].try_into().unwrap();

    // Step 2: Sample secret vectors s, e from CBD_η₁(PRF(σ, N)) (FIPS 203 Algorithm 12, line 3-4)
    let mut nonce: u8 = 0;
    let mut s = [Poly::new(); K];
    let mut e = [Poly::new(); K];
    for i in 0..K {
        let needed = (if ETA1 == 3 { 192 } else { 128 }) as usize;
        let mut buf = vec![0u8; needed];
        prf(&sigma, nonce, &mut buf);
        nonce = nonce.wrapping_add(1);
        sample_cbd(&buf, ETA1, &mut s[i]);
    }
    for i in 0..K {
        let needed = (if ETA1 == 3 { 192 } else { 128 }) as usize;
        let mut buf = vec![0u8; needed];
        prf(&sigma, nonce, &mut buf);
        nonce = nonce.wrapping_add(1);
        sample_cbd(&buf, ETA1, &mut e[i]);
    }

    // Step 3: Convert s, e to NTT domain (FIPS 203 Algorithm 12, line 5-6)
    for i in 0..K {
        s[i].ntt();
    }
    for i in 0..K {
        e[i].ntt();
    }

    // Step 4: Compute t̂ ← Â ◦ ŝ + ê (FIPS 203 Algorithm 12, line 7-9)
    let mut t_hat = [Poly::new(); K];
    for i in 0..K {
        for j in 0..K {
            let mut aij = Poly::new();
            sample_ntt(&rho, i as u8, j as u8, &mut aij);
            // aij is already in NTT domain per FIPS 203
            let mut tmp = aij;
            tmp.pointwise_mul(&s[j]);
            t_hat[i].add(&tmp);
        }
        // Add e_hat in NTT domain
        t_hat[i].add(&e[i]);
    }

    // Step 5: Pack pk = t̂ || ρ (FIPS 203 Algorithm 12, line 10-11)
    for i in 0..K {
        let mut tmp = [0u8; POLY_BYTES];
        poly_encode(&t_hat[i], &mut tmp);
        pk[i * POLY_BYTES..(i + 1) * POLY_BYTES].copy_from_slice(&tmp);
    }
    pk[K * POLY_BYTES..PUBLIC_KEY_BYTES].copy_from_slice(&rho);

    // Step 6: Pack sk = ŝ (FIPS 203 Algorithm 12, line 12)
    for i in 0..K {
        let mut tmp = [0u8; POLY_BYTES];
        poly_encode(&s[i], &mut tmp);
        sk[i * POLY_BYTES..(i + 1) * POLY_BYTES].copy_from_slice(&tmp);
    }
}

#[cfg(test)]
pub fn indcpa_keypair_deterministic(
    d: &[u8; 32],
    pk: &mut [u8; PUBLIC_KEY_BYTES],
    sk: &mut [u8; K * POLY_BYTES]
) {
    // Deterministic key generation for testing (FIPS 203 Algorithm 12 with fixed seed)
    // ρ || σ = G(d || k)
    let mut in33 = [0u8; 33];
    in33[..32].copy_from_slice(d);
    in33[32] = K as u8;
    let mut buf64 = [0u8; 64];
    hash_g(&in33, &mut buf64);
    let rho: [u8; 32] = buf64[..32].try_into().unwrap();
    let sigma: [u8; 32] = buf64[32..].try_into().unwrap();

    let mut nonce: u8 = 0;
    let mut s = [Poly::new(); K];
    let mut e = [Poly::new(); K];
    for i in 0..K {
        let needed = (if ETA1 == 3 { 192 } else { 128 }) as usize;
        let mut buf = vec![0u8; needed];
        prf(&sigma, nonce, &mut buf);
        nonce = nonce.wrapping_add(1);
        sample_cbd(&buf, ETA1, &mut s[i]);
    }
    for i in 0..K {
        let needed = (if ETA1 == 3 { 192 } else { 128 }) as usize;
        let mut buf = vec![0u8; needed];
        prf(&sigma, nonce, &mut buf);
        nonce = nonce.wrapping_add(1);
        sample_cbd(&buf, ETA1, &mut e[i]);
    }

    for i in 0..K {
        s[i].ntt();
    }
    for i in 0..K {
        e[i].ntt();
    }

    let mut t_hat = [Poly::new(); K];
    for i in 0..K {
        for j in 0..K {
            let mut aij = Poly::new();
            sample_ntt(&rho, i as u8, j as u8, &mut aij);
            let mut tmp = aij;
            tmp.pointwise_mul(&s[j]);
            t_hat[i].add(&tmp);
        }
        t_hat[i].add(&e[i]);
    }

    for i in 0..K {
        let mut tmp = [0u8; POLY_BYTES];
        poly_encode(&t_hat[i], &mut tmp);
        pk[i * POLY_BYTES..(i + 1) * POLY_BYTES].copy_from_slice(&tmp);
    }
    pk[K * POLY_BYTES..PUBLIC_KEY_BYTES].copy_from_slice(&rho);

    for i in 0..K {
        let mut tmp = [0u8; POLY_BYTES];
        poly_encode(&s[i], &mut tmp);
        sk[i * POLY_BYTES..(i + 1) * POLY_BYTES].copy_from_slice(&tmp);
    }
}
