use crate::params::{K, N, ETA1, POLY_BYTES, PUBLIC_KEY_BYTES};
use crate::poly::Poly;
use crate::utils::{g, prf, cbd_eta, rej_uniform, xof_matrix, poly_to_bytes};

use rand_core::{CryptoRng, RngCore};

/// IND-CPA keypair generation.
/// Fills `pk` and the first K*POLY_BYTES of `sk` with the secret vector `s`.
/// (The caller `kem::keygen` appends pk || H(pk) || z to the secret key.)
pub fn indcpa_keypair<R: RngCore + CryptoRng>(
    rng: &mut R,
    pk: &mut [u8; PUBLIC_KEY_BYTES],
    sk: &mut [u8; K * POLY_BYTES],
) {
    // 1) Seed -> rho || sigma
    let mut seed = [0u8; 32];
    rng.fill_bytes(&mut seed);
    let mut buf64 = [0u8; 64];
    g(&seed, &mut buf64);
    let rho: [u8; 32] = buf64[..32].try_into().unwrap();
    let sigma: [u8; 32] = buf64[32..].try_into().unwrap();

    // 2) Sample secret s and error e from PRF(sigma, nonce)
    let mut nonce: u8 = 0;
    let mut s = [Poly::new(); K];
    let mut e = [Poly::new(); K];

    for i in 0..K {
        let mut buf = [0u8; 64];
        prf(&sigma, nonce, &mut buf);
        nonce = nonce.wrapping_add(1);
        cbd_eta(&buf, ETA1, &mut s[i]);
    }
    for i in 0..K {
        let mut buf = [0u8; 64];
        prf(&sigma, nonce, &mut buf);
        nonce = nonce.wrapping_add(1);
        cbd_eta(&buf, ETA1, &mut e[i]);
    }

    // NTT(s)
    for i in 0..K {
        s[i].ntt();
    }

    // 3) t = A * s + e
    let mut t = [Poly::new(); K];
    for i in 0..K {
        for j in 0..K {
            let mut stream = [0u8; 3 * N];
            xof_matrix(&rho, i as u8, j as u8, &mut stream);
            let mut aij = Poly::new();
            rej_uniform(&stream, &mut aij);
            aij.ntt();
            let mut tmp = aij;
            tmp.pointwise_mul(&s[j]);
            t[i].add(&tmp);
        }
        t[i].inv_ntt();
        t[i].add(&e[i]);
    }

    // 4) Pack pk = t || rho
    for i in 0..K {
        let mut tmp = [0u8; POLY_BYTES];
        poly_to_bytes(&t[i], &mut tmp);
        pk[i * POLY_BYTES..(i + 1) * POLY_BYTES].copy_from_slice(&tmp);
    }
    pk[K * POLY_BYTES..PUBLIC_KEY_BYTES].copy_from_slice(&rho);

    // 5) Pack sk head = s
    for i in 0..K {
        let mut tmp = [0u8; POLY_BYTES];
        // s is already in NTT domain, but stored like normal; pack as-is
        // (Kyber packs s as 12-bit coefficients; arithmetic uses NTT later)
        // If you prefer, you can store s in normal domain; both forms are acceptable
        // as long as decapsulation mirrors it. Our decaps expects this layout.
        // So we inverse-NTT before storing to match that expectation:
        let mut s_norm = s[i];
        s_norm.inv_ntt();
        poly_to_bytes(&s_norm, &mut tmp);
        sk[i * POLY_BYTES..(i + 1) * POLY_BYTES].copy_from_slice(&tmp);
    }
}
