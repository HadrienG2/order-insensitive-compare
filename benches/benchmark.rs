use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use order_insensitive_compare::{
    ahash_par, ahash_seq, blake3_par, blake3_seq, eq_by_ahash_par, eq_by_ahash_seq,
    eq_by_blake3_par, eq_by_blake3_seq, eq_by_sha256_par, eq_by_sha256_seq, eq_by_sorting_par,
    eq_by_sorting_seq, sha256_par, sha256_seq,
};
use rand::prelude::*;
use rayon::prelude::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    const FILE_SIZE: usize = 60 * 1024 * 1024;
    const NUM_ENTRIES: usize = 1000;
    const ENTRY_SIZE: usize = FILE_SIZE / NUM_ENTRIES;

    let mut data = vec![vec![0; ENTRY_SIZE]; NUM_ENTRIES];
    data.par_iter_mut().for_each(|entry| {
        let mut rng = rand::thread_rng();
        rng.fill_bytes(&mut entry[..]);
    });

    c.bench_function("seq ahash", |b| {
        b.iter_batched(
            || data.clone(),
            |data| ahash_seq(data),
            criterion::BatchSize::LargeInput,
        );
    });

    c.bench_function("seq sha256", |b| {
        b.iter_batched(
            || data.clone(),
            |data| sha256_seq(data),
            criterion::BatchSize::LargeInput,
        );
    });

    c.bench_function("seq blake3", |b| {
        b.iter_batched(
            || data.clone(),
            |data| blake3_seq(data),
            criterion::BatchSize::LargeInput,
        );
    });

    c.bench_function("par ahash", |b| {
        b.iter_batched(
            || data.clone(),
            |data| ahash_par(data),
            criterion::BatchSize::LargeInput,
        );
    });

    c.bench_function("par sha256", |b| {
        b.iter_batched(
            || data.clone(),
            |data| sha256_par(data),
            criterion::BatchSize::LargeInput,
        );
    });

    c.bench_function("par blake3", |b| {
        b.iter_batched(
            || data.clone(),
            |data| blake3_par(data),
            criterion::BatchSize::LargeInput,
        );
    });

    let mut rng = rand::thread_rng();
    let mut shuffled = data.clone();
    shuffled.shuffle(&mut rng);

    c.bench_function("seq compare via sorting", |b| {
        b.iter_batched(
            || (data.clone(), shuffled.clone()),
            |(data, shuffled)| eq_by_sorting_seq(data, shuffled),
            BatchSize::LargeInput,
        );
    });

    c.bench_function("seq compare via ahash", |b| {
        b.iter_batched(
            || (data.clone(), shuffled.clone()),
            |(data, shuffled)| eq_by_ahash_seq(data, shuffled),
            BatchSize::LargeInput,
        );
    });

    c.bench_function("seq compare via sha256", |b| {
        b.iter_batched(
            || (data.clone(), shuffled.clone()),
            |(data, shuffled)| eq_by_sha256_seq(data, shuffled),
            BatchSize::LargeInput,
        );
    });

    c.bench_function("seq compare via blake3", |b| {
        b.iter_batched(
            || (data.clone(), shuffled.clone()),
            |(data, shuffled)| eq_by_blake3_seq(data, shuffled),
            BatchSize::LargeInput,
        );
    });

    c.bench_function("par compare via sorting", |b| {
        b.iter_batched(
            || (data.clone(), shuffled.clone()),
            |(data, shuffled)| eq_by_sorting_par(data, shuffled),
            BatchSize::LargeInput,
        );
    });

    c.bench_function("par compare via ahash", |b| {
        b.iter_batched(
            || (data.clone(), shuffled.clone()),
            |(data, shuffled)| eq_by_ahash_par(data, shuffled),
            BatchSize::LargeInput,
        );
    });

    c.bench_function("par compare via sha256", |b| {
        b.iter_batched(
            || (data.clone(), shuffled.clone()),
            |(data, shuffled)| eq_by_sha256_par(data, shuffled),
            BatchSize::LargeInput,
        );
    });

    c.bench_function("par compare via blake3", |b| {
        b.iter_batched(
            || (data.clone(), shuffled.clone()),
            |(data, shuffled)| eq_by_blake3_par(data, shuffled),
            BatchSize::LargeInput,
        );
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
