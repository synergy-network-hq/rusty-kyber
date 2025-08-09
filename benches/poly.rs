use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use rand_chacha::ChaCha20Rng;
use rand_core::{RngCore, SeedableRng};

use rusty_kyber::params::{N, Q};
use rusty_kyber::poly::Poly;

fn make_poly(seed: [u8; 32]) -> Poly {
    let mut rng = ChaCha20Rng::from_seed(seed);
    let mut p = Poly::new();
    for i in 0..N {
        p.coeffs[i] = (rng.next_u32() as i32 % Q) as i16;
    }
    p
}

fn bench_poly(c: &mut Criterion) {
    let mut group = c.benchmark_group("poly");
    group.throughput(Throughput::Elements(N as u64));

    group.bench_function("add", |b| {
        let a = make_poly([0xAA; 32]);
        let bpoly = make_poly([0xBB; 32]);
        b.iter_batched(
            || (a, bpoly),
            |(mut x, y)| {
                x.add(&y);
                black_box(x)
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("sub", |b| {
        let a = make_poly([0xCC; 32]);
        let bpoly = make_poly([0xDD; 32]);
        b.iter_batched(
            || (a, bpoly),
            |(mut x, y)| {
                x.sub(&y);
                black_box(x)
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(benches, bench_poly);
criterion_main!(benches);
