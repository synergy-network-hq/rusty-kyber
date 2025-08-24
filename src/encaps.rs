use crate::params::{ K, PUBLIC_KEY_BYTES, CIPHERTEXT_BYTES, POLY_BYTES, DU, DV, ETA1, ETA2 };
use crate::math::{
    Poly,
    prf,
    sample_cbd,
    sample_ntt,
    poly_decode,
    poly_from_msg,
    poly_compress_u,
    poly_compress_v,
};

pub fn indcpa_enc(
    pk: &[u8; PUBLIC_KEY_BYTES],
    msg: &[u8; 32],
    coins: &[u8; 32],
    ct: &mut [u8; CIPHERTEXT_BYTES]
) {
    // Unpack t and rho from pk
    let mut t = [Poly::new(); K];
    for i in 0..K {
        let mut tmp = [0u8; POLY_BYTES];
        tmp.copy_from_slice(&pk[i * POLY_BYTES..(i + 1) * POLY_BYTES]);
        poly_decode(&tmp, &mut t[i]);
        // t_hat is stored in NTT domain, so we need to convert back to NTT domain
        t[i].ntt();
    }
    let rho: &[u8; 32] = pk[K * POLY_BYTES..PUBLIC_KEY_BYTES].try_into().unwrap();

    // Sample r, e1, e2 from coins via PRF+CBD
    let mut nonce: u8 = 0;
    let mut rvec = [Poly::new(); K];
    let mut e1 = [Poly::new(); K];
    for i in 0..K {
        let needed = (if ETA1 == 3 { 192 } else { 128 }) as usize;
        let mut buf = vec![0u8; needed];
        prf(coins, nonce, &mut buf);
        nonce = nonce.wrapping_add(1);
        sample_cbd(&buf[..], ETA1, &mut rvec[i]);
    }
    for i in 0..K {
        let mut buf = vec![0u8; 128];
        prf(coins, nonce, &mut buf);
        nonce = nonce.wrapping_add(1);
        sample_cbd(&buf[..], ETA2, &mut e1[i]);
    }

    let mut e2 = Poly::new();
    let mut buf = vec![0u8; 128];
    prf(coins, nonce, &mut buf);
    sample_cbd(&buf[..], ETA2, &mut e2);

    // u = InvNTT( A_hat * NTT(r) ) + e1
    let mut u = [Poly::new(); K];
    for i in 0..K {
        let mut acc = Poly::new();
        for j in 0..K {
            let mut aij = Poly::new();
            sample_ntt(rho, i as u8, j as u8, &mut aij);
            // aij is already in NTT domain per FIPS 203
            let mut rj = rvec[j];
            rj.ntt();
            let mut tmp = aij;
            tmp.pointwise_mul(&rj);
            acc.add(&tmp);
        }
        acc.inv_ntt();
        acc.add(&e1[i]);
        u[i] = acc;
    }

    // v = InvNTT( t_hat^T * NTT(r) ) + e2 + m
    let mut v = Poly::new();
    for j in 0..K {
        // t[j] is already in NTT domain (t_hat) when read from pk
        let tj = t[j];
        let mut rj = rvec[j];
        rj.ntt();
        let mut tmp = tj;
        tmp.pointwise_mul(&rj);
        v.add(&tmp);
    }
    v.inv_ntt();

    let mut mpoly = Poly::new();
    poly_from_msg(msg, &mut mpoly);
    v.add(&e2);
    v.add(&mpoly);

    // Pack ciphertext: u (K polys, DU bits) || v (1 poly, DV bits)
    let mut offset = 0usize;
    for i in 0..K {
        let bytes = (DU * 256) / 8;
        poly_compress_u(&u[i], &mut ct[offset..offset + bytes]);
        offset += bytes;
    }
    let vbytes = (DV * 256) / 8;
    poly_compress_v(&v, &mut ct[offset..offset + vbytes]);
}
