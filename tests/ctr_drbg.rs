//! Minimal SP 800-90A CTR_DRBG (AES-256) for NIST KATs.
//! Instantiate with a 48-byte seed. Implements RngCore + CryptoRng.

use aes::cipher::{generic_array::GenericArray, BlockEncrypt, KeyInit};
use aes::Aes256;
use rand_core::{CryptoRng, Error, RngCore};

pub struct CtrDrbg {
    key: [u8; 32],
    v: [u8; 16],
}

impl CtrDrbg {
    /// Instantiate per SP 800-90A without DF:
    ///   Key = 0^256, V = 0^128; then Update(seed_material)
    /// `seed` must be exactly 48 bytes.
    pub fn new(seed: &[u8]) -> Result<Self, String> {
        if seed.len() != 48 {
            return Err(format!("CTR_DRBG seed must be 48 bytes, got {}", seed.len()));
        }
        let mut drbg = Self { key: [0u8; 32], v: [0u8; 16] };
        drbg.update(Some(seed));
        Ok(drbg)
    }

    #[inline]
    fn encrypt_block(key: &[u8; 32], input: &[u8; 16]) -> [u8; 16] {
        let cipher = Aes256::new(GenericArray::from_slice(key));
        let mut block = GenericArray::clone_from_slice(input);
        cipher.encrypt_block(&mut block);
        let mut out = [0u8; 16];
        out.copy_from_slice(&block);
        out
    }

    #[inline]
    fn inc_be_128(x: &mut [u8; 16]) {
        for i in (0..16).rev() {
            let (v, c) = x[i].overflowing_add(1);
            x[i] = v;
            if !c {
                break;
            }
        }
    }

    /// SP 800-90A `Update` function. If `provided` is Some(48),
    /// XOR it into the generated temp before setting Key/V.
    fn update(&mut self, provided: Option<&[u8]>) {
        let mut temp = [0u8; 48];
        let mut v = self.v;

        // Generate 48 bytes: encrypt V+1, V+2, V+3 under current Key.
        for chunk in temp.chunks_mut(16) {
            Self::inc_be_128(&mut v);
            let block = Self::encrypt_block(&self.key, &v);
            chunk.copy_from_slice(&block);
        }

        if let Some(pd) = provided {
            assert_eq!(pd.len(), 48, "provided_data must be 48 bytes");
            for (t, p) in temp.iter_mut().zip(pd.iter()) {
                *t ^= *p;
            }
        }

        // Split into new Key || V
        self.key.copy_from_slice(&temp[..32]);
        self.v.copy_from_slice(&temp[32..48]);
    }
}

impl RngCore for CtrDrbg {
    fn next_u32(&mut self) -> u32 {
        let mut buf = [0u8; 4];
        self.fill_bytes(&mut buf);
        u32::from_le_bytes(buf)
    }

    fn next_u64(&mut self) -> u64 {
        let mut buf = [0u8; 8];
        self.fill_bytes(&mut buf);
        u64::from_le_bytes(buf)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut off = 0usize;
        let mut v = self.v;

        while off < dest.len() {
            Self::inc_be_128(&mut v);
            let block = Self::encrypt_block(&self.key, &v);
            let take = core::cmp::min(16, dest.len() - off);
            dest[off..off + take].copy_from_slice(&block[..take]);
            off += take;
        }

        // Update internal state after each generate
        self.v = v;
        self.update(None);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl CryptoRng for CtrDrbg {}
