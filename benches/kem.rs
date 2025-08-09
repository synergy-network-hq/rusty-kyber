use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use rand_chacha::ChaCha20Rng;
use rand_core::SeedableRng;

use rusty_kyber::{decapsulate, encapsulate, keypair};

fn bench_kem(c: &mut Criterion) {
    // Deterministic RNGs for stable results
    let mut group = c.benchmark_group("kem");

    group.bench_function("keypair", |b| {
        b.iter_batched(
            || ChaCha20Rng::from_seed([0x11; 32]),
            |mut rng| {
                let (pk, sk) = keypair(&mut rng);
                black_box((pk, sk))
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("encapsulate", |b| {
        let mut rng = ChaCha20Rng::from_seed([0x22; 32]);
        let (pk, _sk) = keypair(&mut rng);
        b.iter_batched(
            || ChaCha20Rng::from_seed([0x33; 32]),
            |mut rng_ct| {
                let (ct, ss) = encapsulate(&mut rng_ct, &pk);
                black_box((ct, ss))
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("decapsulate", |b| {
        let mut rng = ChaCha20Rng::from_seed([0x44; 32]);
        let (pk, sk) = keypair(&mut rng);
        let (ct, _ss) = encapsulate(&mut rng, &pk);
        b.iter(|| {
            let ss2 = decapsulate(&sk, &ct);
            black_box(ss2)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_kem);
criterion_main!(benches);
