FIPS 203 conformance notes and corrections

* XOF index order for Â[i, j]
  + Issue: Previously absorbed `rho || i || j`. FIPS 203 requires `rho || j || i`.
  + Fix: `src/utils.rs::xof_matrix` now uses `[j, i]`. See FIPS 203 Appendix C.2.
  + Impact: Affects Â generation for keygen/encaps; KATs depend on this.

* Inconsistent hashing API style
  + Issue: Mixed return-value and out-parameter variants for `h`,   `g`,   `kdf`,  `prf` causing duplicate/contradictory symbols.
  + Fix: Standardize to out-parameter style per crate conventions (`h/g/kdf/prf`).
  + Files: `src/utils.rs`, callsites updated across crate.

* CBD buffer sizes (PRFη output length)
  + Issue: Used fixed 64-byte PRF output; FIPS 203 requires 128 bytes (η=2) and 192 bytes (η=3).
  + Fix: `src/keygen.rs` and `src/encaps.rs` allocate 128/192 appropriately before `cbd_eta`.

* e1 sampling parameter
  + Issue: e1 was previously sampled with `ETA1`; per ML-KEM, e1 uses η2.
  + Fix: `src/encaps.rs` now samples `e1` with `ETA2` and `e2` with `ETA2`.

* NTT accumulation in decapsulation
  + Issue: Decapsulation subtracted per-term during NTT loop; restructured to sum products in NTT domain, inverse-NTT once, then subtract from `v` for clarity and alignment.
  + Fix: `src/decaps.rs` computes `sum_i NTT(u_i)⋅s_i`, then `InvNTT` and subtracts from `v`.

* Polynomial 12-bit codec
  + Issue: Overhaul to ensure no out-of-bounds writes and exact 12-byte-per-8-coeff packing.
  + Fix: `poly_to_bytes` and `poly_from_bytes` updated to 12-byte layout and masking to 12 bits.

Reference: FIPS 203 ML-KEM, Appendix A/B/C and relevant algorithm sections ( `https://nvlpubs.nist.gov/nistpubs/fips/nist.fips.203.pdf` ).
