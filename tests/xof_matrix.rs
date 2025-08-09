use rusty_kyber::params::{K, N, Q};
use rusty_kyber::poly::Poly;
use rusty_kyber::utils::{rej_uniform, xof_matrix};

fn within_mod_q(p: &Poly) -> bool {
    p.coeffs.iter().all(|&c| (c as i32) >= 0 && (c as i32) < Q)
}

#[test]
fn xof_matrix_repro_and_range() {
    // For a fixed rho, A[i,j] must be reproducible and lead to coeffs in [0,q).
    let rho = [0x42u8; 32];

    // Sample two positions in A: (0,0) and (K-1, K-1)
    for &(i, j) in &[(0u8, 0u8), ((K - 1) as u8, (K - 1) as u8)] {
        // Generate more than enough bytes for rejection sampling (12 bits per coeff -> ~1.5 bytes avg)
        let mut stream = vec![0u8; 3 * N]; // comfortably sufficient for a full poly
        xof_matrix(&rho, i, j, &mut stream);

        let mut p1 = Poly::new();
        rej_uniform(&stream, &mut p1);

        // Re-run with the same inputs must give identical poly
        let mut stream2 = vec![0u8; 3 * N];
        xof_matrix(&rho, i, j, &mut stream2);
        let mut p2 = Poly::new();
        rej_uniform(&stream2, &mut p2);

        assert_eq!(p1.coeffs, p2.coeffs, "A[{},{}] not reproducible", i, j);
        assert!(within_mod_q(&p1), "coeff out of [0,q)");
    }
}
