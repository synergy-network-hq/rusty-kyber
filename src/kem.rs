use crate::encaps::indcpa_enc;
use crate::decaps::indcpa_dec;
use crate::keygen::indcpa_keypair;
use crate::params::{
    CIPHERTEXT_BYTES, K, POLY_BYTES, PUBLIC_KEY_BYTES, SECRET_KEY_BYTES, SHARED_SECRET_BYTES,
};
use crate::utils::{g, h, kdf};
use rand_core::{CryptoRng, RngCore};
use subtle::{Choice, ConditionallySelectable};

fn ct_eq_slices(a: &[u8], b: &[u8]) -> Choice {
    debug_assert_eq!(a.len(), b.len());
    let mut diff: u8 = 0;
    for i in 0..a.len() {
        diff |= a[i] ^ b[i];
    }
    let nz = ((diff | diff.wrapping_neg()) >> 7) & 1;
    Choice::from(1 ^ nz)
}

pub fn keygen<R: RngCore + CryptoRng>(
    rng: &mut R,
    pk: &mut [u8; PUBLIC_KEY_BYTES],
    sk: &mut [u8; SECRET_KEY_BYTES],
) {
    // IND-CPA keypair -> t || rho in pk, s in sk_head
    let mut sk_head = [0u8; K * POLY_BYTES];
    indcpa_keypair(rng, pk, &mut sk_head);

    // Append pk, H(pk), z to secret key (layout: s || pk || H(pk) || z)
    let mut hpk = [0u8; 32];
    h(pk, &mut hpk);

    let mut z = [0u8; 32];
    rng.fill_bytes(&mut z);

    let mut off = 0usize;
    sk[off..off + K * POLY_BYTES].copy_from_slice(&sk_head);
    off += K * POLY_BYTES;
    sk[off..off + PUBLIC_KEY_BYTES].copy_from_slice(pk);
    off += PUBLIC_KEY_BYTES;
    sk[off..off + 32].copy_from_slice(&hpk);
    off += 32;
    sk[off..off + 32].copy_from_slice(&z);
}

pub fn encaps<R: RngCore + CryptoRng>(
    rng: &mut R,
    pk: &[u8; PUBLIC_KEY_BYTES],
    ss: &mut [u8; SHARED_SECRET_BYTES],
    ct: &mut [u8; CIPHERTEXT_BYTES],
) {
    let mut rnd = [0u8; 32];
    rng.fill_bytes(&mut rnd);
    let mut m = [0u8; 32];
    h(&rnd, &mut m);

    let mut hpk = [0u8; 32];
    h(pk, &mut hpk);

    let mut g_in = [0u8; 64];
    g_in[..32].copy_from_slice(&m);
    g_in[32..].copy_from_slice(&hpk);

    let mut kr = [0u8; 64];
    g(&g_in, &mut kr);
    let (k, coins) = kr.split_at(32);

    indcpa_enc(pk, &m, coins.try_into().unwrap(), ct);

    let mut hc = [0u8; 32];
    h(ct, &mut hc);

    let mut kdf_in = [0u8; 64];
    kdf_in[..32].copy_from_slice(k);
    kdf_in[32..].copy_from_slice(&hc);
    kdf(&kdf_in, ss);
}

pub fn decaps(
    sk: &[u8; SECRET_KEY_BYTES],
    ct: &[u8; CIPHERTEXT_BYTES],
    ss: &mut [u8; SHARED_SECRET_BYTES],
) {
    // Unpack pk, H(pk), z from sk
    let mut off = K * POLY_BYTES;
    let pk: &[u8] = &sk[off..off + PUBLIC_KEY_BYTES];
    off += PUBLIC_KEY_BYTES;
    let hpk: &[u8] = &sk[off..off + 32];
    off += 32;
    let z: &[u8] = &sk[off..off + 32];

    // m' = dec(sk, ct)  — your indcpa_dec expects the full secret key buffer
    let mut m = [0u8; 32];
    indcpa_dec(sk, ct, &mut m);

    // kr' = G(m' || H(pk))
    let mut g_in = [0u8; 64];
    g_in[..32].copy_from_slice(&m);
    g_in[32..].copy_from_slice(hpk);
    let mut kr = [0u8; 64];
    g(&g_in, &mut kr);
    let (k, coins) = kr.split_at(32);

    // Re-encrypt to check
    let mut ct2 = [0u8; CIPHERTEXT_BYTES];
    let pk_fixed: &[u8; PUBLIC_KEY_BYTES] = pk.try_into().unwrap();
    indcpa_enc(pk_fixed, &m, coins.try_into().unwrap(), &mut ct2);

    // Constant-time select: if ct == ct2 choose k else z
    let equal = ct_eq_slices(ct, &ct2);
    let mut sel = [0u8; 32];
    for i in 0..32 {
        sel[i] = u8::conditional_select(&z[i], &k[i], equal);
    }

    let mut hc = [0u8; 32];
    h(ct, &mut hc);
    let mut kdf_in = [0u8; 64];
    kdf_in[..32].copy_from_slice(&sel);
    kdf_in[32..].copy_from_slice(&hc);
    kdf(&kdf_in, ss);
}
