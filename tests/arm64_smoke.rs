use rand::rngs::OsRng;
use rusty_kyber::{keypair, encapsulate, decapsulate};

fn main() {
    // Tiny runtime smoke (used under cross + qemu-aarch64 in CI)
    let (pk, sk) = keypair(&mut OsRng);
    let (ct, ss_sender) = encapsulate(&mut OsRng, &pk);
    let ss_receiver = decapsulate(&sk, &ct);
    assert_eq!(ss_sender.as_bytes(), ss_receiver.as_bytes());
    println!("arm64 smoke ok, ss[0..4]={:02x?}", &ss_receiver.as_bytes()[0..4]);
}
