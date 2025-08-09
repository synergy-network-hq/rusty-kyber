use crate::params::{
    K, N, PUBLIC_KEY_BYTES, CIPHERTEXT_BYTES, POLY_BYTES, DU, DV, ETA1, ETA2,
};
use crate::poly::Poly;
use crate::utils::{
    xof_matrix, prf, cbd_eta, rej_uniform, poly_from_bytes, poly_from_msg,
    poly_compress_u, poly_compress_v,
};

pub fn indcpa_enc(
    pk: &[u8; PUBLIC_KEY_BYTES],
    msg: &[u8; 32],
    coins: &[u8; 32],
    ct: &mut [u8; CIPHERTEXT_BYTES],
) {
    // Unpack t and rho from pk
    let mut t = [Poly::new(); K];
    for i in 0..K {
        let mut tmp = [0u8; POLY_BYTES];
        tmp.copy_from_slice(&pk[i * POLY_BYTES..(i + 1) * POLY_BYTES]);
        poly_from_bytes(&tmp, &mut t[i]);
    }
    let rho: &[u8; 32] = pk[K * POLY_BYTES..PUBLIC_KEY_BYTES].try_into().unwrap();

    // Sample r, e1, e2 from coins via PRF+CBD
    let mut nonce: u8 = 0;
    let mut rvec = [Poly::new(); K];
    let mut e1 = [Poly::new(); K];
    for i in 0..K {
        let mut buf = [0u8; 64];
        prf(coins, nonce, &mut buf);
        nonce = nonce.wrapping_add(1);
        cbd_eta(&buf[..], ETA1, &mut rvec[i]);
        // Reduce into NTT domain later via pointwise mul schedule
    }
    let mut e2 = Poly::new();
    let mut buf = [0u8; 64];
    prf(coins, nonce, &mut buf);
    cbd_eta(&buf[..], ETA2, &mut e2);

    // u = A * r + e1
    let mut u = [Poly::new(); K];
    for i in 0..K {
        for j in 0..K {
            let mut stream = [0u8; 3 * N]; // 12-bit rejection sampling reservoir
            xof_matrix(rho, i as u8, j as u8, &mut stream);
            let mut aij = Poly::new();
            rej_uniform(&stream, &mut aij);
            aij.ntt();

            let mut tmp = aij;
            tmp.pointwise_mul(&rvec[j]);
            u[i].add(&tmp);
        }
        u[i].inv_ntt();
        u[i].add(&e1[i]);
    }

    // v = t^T * r + e2 + m
    let mut v = Poly::new();
    for j in 0..K {
        let mut tj = t[j];
        tj.ntt();
        let mut tmp = tj;
        tmp.pointwise_mul(&rvec[j]);
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
        let bytes = DU * 256 / 8;
        poly_compress_u(&u[i], &mut ct[offset..offset + bytes]);
        offset += bytes;
    }
    let vbytes = DV * 256 / 8;
    poly_compress_v(&v, &mut ct[offset..offset + vbytes]);
}
