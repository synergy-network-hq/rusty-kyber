use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use rand_chacha::ChaCha20Rng;
use rand_core::{RngCore, SeedableRng};

use rusty_kyber::params::{N, Q};
use rusty_kyber::poly::{inv_ntt_batch, ntt_batch, Poly};

const COUNT: usize = 256; // number of polys per bulk run

fn make_polys() -> Vec<Poly> {
    let mut rng = ChaCha20Rng::from_seed([0x42; 32]);
    let mut v = Vec::with_capacity(COUNT);
    for _ in 0..COUNT {
        let mut p = Poly::new();
        for i in 0..N {
            p.coeffs[i] = (rng.next_u32() as i32 % Q) as i16;
        }
        v.push(p);
    }
    v
}

fn bench_ntt(c: &mut Criterion) {
    let mut group = c.benchmark_group("ntt");
    group.throughput(Throughput::Elements((COUNT * N) as u64));

    // Single NTT: loop per poly
    group.bench_function("ntt_single_loop", |b| {
        let base = make_polys();
        b.iter_batched(
            || base.clone(),
            |mut polys| {
                for p in polys.iter_mut() {
                    p.ntt();
                }
                black_box(polys[0].coeffs[0])
            },
            BatchSize::LargeInput,
        );
    });

    // Batched NTT
    group.bench_function("ntt_batched", |b| {
        let base = make_polys();
        b.iter_batched(
            || base.clone(),
            |mut polys| {
                ntt_batch(&mut polys[..]);
                black_box(polys[0].coeffs[0])
            },
            BatchSize::LargeInput,
        );
    });

    // Single InvNTT: loop per poly
    group.bench_function("inv_ntt_single_loop", |b| {
        let base = {
            let mut v = make_polys();
            for p in v.iter_mut() { p.ntt(); }
            v
        };
        b.iter_batched(
            || base.clone(),
            |mut polys| {
                for p in polys.iter_mut() {
                    p.inv_ntt();
                }
                black_box(polys[0].coeffs[0])
            },
            BatchSize::LargeInput,
        );
    });

    // Batched InvNTT
    group.bench_function("inv_ntt_batched", |b| {
        let base = {
            let mut v = make_polys();
            for p in v.iter_mut() { p.ntt(); }
            v
        };
        b.iter_batched(
            || base.clone(),
            |mut polys| {
                inv_ntt_batch(&mut polys[..]);
                black_box(polys[0].coeffs[0])
            },
            BatchSize::LargeInput,
        );
    });

    group.finish();
}

criterion_group!(benches, bench_ntt);
criterion_main!(benches);
