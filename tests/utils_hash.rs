use sha3::{Digest, Sha3_256, Sha3_512, Shake128, Shake256};
use sha3::digest::{ExtendableOutput, Update, XofReader};

use rusty_kyber::params::{N, Q};
use rusty_kyber::poly::Poly;
use rusty_kyber::utils::{h, g, kdf, prf, xof_matrix, rej_uniform};

#[test]
fn h_matches_sha3_256() {
    let msg = b"sample message for sha3-256";
    let mut ours = [0u8; 32];
    h(msg, &mut ours);

    let mut hasher = Sha3_256::new();
    hasher.update(msg);
    let expected = hasher.finalize();

    assert_eq!(ours.as_slice(), expected.as_slice());
}

#[test]
fn g_matches_sha3_512() {
    let msg = b"sample message for sha3-512";
    let mut ours = [0u8; 64];
    g(msg, &mut ours);

    let mut hasher = Sha3_512::new();
    hasher.update(msg);
    let expected = hasher.finalize();

    assert_eq!(ours.as_slice(), expected.as_slice());
}

#[test]
fn kdf_matches_shake256_32bytes() {
    let input = b"kdf-test-input";
    let mut ours = [0u8; 32];
    kdf(input, &mut ours);

    let mut x = Shake256::default();
    x.update(input);
    let mut rdr = x.finalize_xof();
    let mut expected = [0u8; 32];
    rdr.read(&mut expected);

    assert_eq!(ours, expected);
}

#[test]
fn prf_nonce_and_length_behavior() {
    let seed = [0xA5u8; 32];
    let mut out32_a = [0u8; 32];
    let mut out32_b = [0u8; 32];
    prf(&seed, 7, &mut out32_a);
    prf(&seed, 8, &mut out32_b);
    assert_ne!(out32_a, out32_b); // different nonces => different output

    let mut out64 = [0u8; 64];
    prf(&seed, 7, &mut out64);
    assert_eq!(&out64[..32], &out32_a[..]); // first 32 bytes match
}

#[test]
fn xof_matrix_equivalence_to_shake128_concat() {
    let rho = [0x11u8; 32];
    let (i, j) = (3u8, 5u8);

    let mut ours = [0u8; 64];
    xof_matrix(&rho, i, j, &mut ours);

    // Direct SHAKE128 over rho || [i, j]
    let mut x = Shake128::default();
    x.update(&rho);
    x.update(&[i, j]);
    let mut rdr = x.finalize_xof();
    let mut expected = [0u8; 64];
    rdr.read(&mut expected);

    assert_eq!(ours, expected);
}

#[test]
fn rej_uniform_exact_pack() {
    // Build a buffer encoding exactly N 12-bit values in the layout expected by rej_uniform.
    // Pack two 12-bit values (v0, v1) into 3 bytes: see utils.rs for the inverse.
    let mut vals = vec![0u16; N];
    for i in 0..N {
        vals[i] = (i as u16) % (Q as u16);
    }

    let mut buf = vec![0u8; 3 * (N / 2)];
    for idx in 0..(N / 2) {
        let v0 = vals[2 * idx];
        let v1 = vals[2 * idx + 1];

        buf[3 * idx] = (v0 & 0xFF) as u8;
        buf[3 * idx + 1] = ((v0 >> 8) as u8) & 0x0F | (((v1 & 0x0F) as u8) << 4);
        buf[3 * idx + 2] = (v1 >> 4) as u8;
    }

    let mut p = Poly::new();
    rej_uniform(&buf, &mut p);

    for i in 0..N {
        assert_eq!(p.coeffs[i] as u16, vals[i], "coeff {}", i);
    }
}
