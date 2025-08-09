mod ctr_drbg;

use std::{fs, path::Path};

use ctr_drbg::CtrDrbg;

use rusty_kyber::{
    decapsulate, encapsulate, keypair,
    params::{CIPHERTEXT_BYTES, PUBLIC_KEY_BYTES, SECRET_KEY_BYTES, SHARED_SECRET_BYTES},
};

#[cfg(feature = "std")]
use rusty_kyber::{encapsulate_osrng, keypair_osrng};

fn hex_to_bytes(s: &str) -> Result<Vec<u8>, String> {
    let s = s.trim();
    if s.len() % 2 != 0 {
        return Err("hex string has odd length".into());
    }
    let mut out = Vec::with_capacity(s.len() / 2);
    let bytes = s.as_bytes();
    let h2n = |c: u8| -> Result<u8, String> {
        match c {
            b'0'..=b'9' => Ok(c - b'0'),
            b'a'..=b'f' => Ok(c - b'a' + 10),
            b'A'..=b'F' => Ok(c - b'A' + 10),
            _ => Err(format!("invalid hex char: {}", c as char)),
        }
    };
    for i in (0..bytes.len()).step_by(2) {
        let hi = h2n(bytes[i])?;
        let lo = h2n(bytes[i + 1])?;
        out.push((hi << 4) | lo);
    }
    Ok(out)
}

#[derive(Default)]
struct KatEntry {
    seed: Option<Vec<u8>>,
    pk: Option<Vec<u8>>,
    sk: Option<Vec<u8>>,
    ct: Option<Vec<u8>>,
    ss: Option<Vec<u8>>,
}

fn pick_kat_file() -> &'static str {
    match PUBLIC_KEY_BYTES {
        800 => "tests/kat_vectors/kyber512.rsp",
        1184 => "tests/kat_vectors/kyber768.rsp",
        1568 => "tests/kat_vectors/kyber1024.rsp",
        _ => "tests/kat_vectors/kyber512.rsp",
    }
}

fn parse_kat_file(path: &str) -> Result<Vec<KatEntry>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("read {}: {}", path, e))?;
    let mut out: Vec<KatEntry> = Vec::new();
    let mut cur = KatEntry::default();

    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        // Push when a full record is formed (seed + pk + sk + ct + ss)
        let mut push_if_ready = || {
            if cur.seed.is_some() && cur.pk.is_some() && cur.sk.is_some()
                && cur.ct.is_some() && cur.ss.is_some()
            {
                out.push(KatEntry {
                    seed: cur.seed.take(),
                    pk: cur.pk.take(),
                    sk: cur.sk.take(),
                    ct: cur.ct.take(),
                    ss: cur.ss.take(),
                });
            }
        };

        if let Some(rest) = line.strip_prefix("count =") {
            push_if_ready();
            let _ = rest; // not used
        } else if let Some(rest) = line.strip_prefix("seed =") {
            cur.seed = Some(hex_to_bytes(rest).map_err(|e| format!("seed hex: {}", e))?);
        } else if let Some(rest) = line.strip_prefix("pk =") {
            cur.pk = Some(hex_to_bytes(rest).map_err(|e| format!("pk hex: {}", e))?);
        } else if let Some(rest) = line.strip_prefix("sk =") {
            cur.sk = Some(hex_to_bytes(rest).map_err(|e| format!("sk hex: {}", e))?);
        } else if let Some(rest) = line.strip_prefix("ct =") {
            cur.ct = Some(hex_to_bytes(rest).map_err(|e| format!("ct hex: {}", e))?);
        } else if let Some(rest) = line.strip_prefix("ss =") {
            cur.ss = Some(hex_to_bytes(rest).map_err(|e| format!("ss hex: {}", e))?);
        } else {
            continue;
        }
    }
    // Final record may not be followed by a blank line.
    if cur.seed.is_some() && cur.pk.is_some() && cur.sk.is_some()
        && cur.ct.is_some() && cur.ss.is_some()
    {
        out.push(cur);
    }

    if out.is_empty() {
        return Err("no KAT entries parsed".into());
    }
    Ok(out)
}

#[test]
fn kat_full_vectors_match() {
    let candidate = pick_kat_file();

    // Choose an actual file or bail out of this test early.
    let path = if Path::new(candidate).exists() {
        Some(candidate)
    } else {
        ["tests/kat_vectors/kyber512_clean.rsp",
         "tests/kat_vectors/kyber768_clean.rsp",
         "tests/kat_vectors/kyber1024_clean.rsp"]
        .iter()
        .find(|p| Path::new(p).exists())
        .copied()
    };

    let Some(path) = path else {
        eprintln!("No KAT file found in tests/kat_vectors; skipping full-KAT test.");
        return;
    };

    let entries = match parse_kat_file(path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to parse {}: {}; skipping.", path, e);
            return;
        }
    };

    let mut ran = 0usize;
    for (i, e) in entries.iter().enumerate() {
        let seed = e.seed.as_ref().unwrap();
        let pk = e.pk.as_ref().unwrap();
        let sk = e.sk.as_ref().unwrap();
        let ct = e.ct.as_ref().unwrap();
        let ss = e.ss.as_ref().unwrap();

        // Sanity on sizes for the active parameter set
        if pk.len() != PUBLIC_KEY_BYTES
            || sk.len() != SECRET_KEY_BYTES
            || ct.len() != CIPHERTEXT_BYTES
            || ss.len() != SHARED_SECRET_BYTES
        {
            continue;
        }

        // Instantiate deterministic DRBG for this entry
        let mut drbg = match CtrDrbg::new(seed) {
            Ok(r) => r,
            Err(err) => panic!("bad seed in {} entry {}: {}", path, i, err),
        };

        // Keypair
        let (got_pk, got_sk) = keypair(&mut drbg);

        // Encapsulate with same DRBG stream (per NIST harness)
        let (got_ct, got_ss) = encapsulate(&mut drbg, &got_pk);

        // Expect exact byte-for-byte matches
        assert_eq!(got_pk.as_bytes(), &pk[..], "pk mismatch at entry {} from {}", i, path);
        assert_eq!(got_sk.as_bytes(), &sk[..], "sk mismatch at entry {} from {}", i, path);
        assert_eq!(got_ct.as_bytes(), &ct[..], "ct mismatch at entry {} from {}", i, path);
        assert_eq!(got_ss.as_bytes(), &ss[..], "ss mismatch at entry {} from {}", i, path);

        // And decapsulation roundtrip
        let dec_ss = decapsulate(&got_sk, &got_ct);
        assert_eq!(dec_ss.as_bytes(), got_ss.as_bytes(), "decaps ss mismatch at entry {} from {}", i, path);

        ran += 1;
    }
    assert!(ran > 0, "no matching KAT entries for this parameter set in {}", path);
}

#[cfg(all(test, feature = "std"))]
#[test]
fn roundtrip_encaps_decaps_osrng() {
    let (pk, sk) = keypair_osrng();
    let (ct, ss1) = encapsulate_osrng(&pk);
    let ss2 = rusty_kyber::decapsulate(&sk, &ct);
    assert_eq!(ss1.as_bytes(), ss2.as_bytes());
}
