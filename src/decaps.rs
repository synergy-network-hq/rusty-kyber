//! IND-CPA public-key decapsulation.

use crate::params::{K, SECRET_KEY_BYTES, CIPHERTEXT_BYTES, POLY_BYTES, DU, DV};
use crate::poly::Poly;
use crate::utils::{poly_from_bytes, poly_decompress_u, poly_decompress_v, poly_to_msg};

pub fn indcpa_dec(
    sk: &[u8; SECRET_KEY_BYTES],
    ct: &[u8; CIPHERTEXT_BYTES],
    msg: &mut [u8; 32],
) {
    // Unpack indcpa secret vector s (first K*POLY_BYTES of sk)
    let mut s = [Poly::new(); K];
    for i in 0..K {
        let mut tmp = [0u8; POLY_BYTES];
        tmp.copy_from_slice(&sk[i * POLY_BYTES..(i + 1) * POLY_BYTES]);
        poly_from_bytes(&tmp, &mut s[i]);
        s[i].ntt();
    }

    // Unpack u and v from ciphertext (compressed)
    let mut offset = 0usize;
    let mut u = [Poly::new(); K];
    for i in 0..K {
        let bytes = DU * 256 / 8;
        poly_decompress_u(&ct[offset..offset + bytes], &mut u[i]);
        offset += bytes;
    }
    let mut v = Poly::new();
    let vbytes = DV * 256 / 8;
    poly_decompress_v(&ct[offset..offset + vbytes], &mut v);

    // m' = v − u^T * s
    let mut mp = v;
    for i in 0..K {
        let mut ui = u[i];
        ui.ntt();
        let mut tmp = ui;
        tmp.pointwise_mul(&s[i]);
        mp.sub(&tmp);
    }
    mp.inv_ntt();

    // Decode 32-byte message
    poly_to_msg(&mp, msg);
}
