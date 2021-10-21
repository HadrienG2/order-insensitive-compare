use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use order_insensitive_compare::{
    eq_by_hashing_par, eq_by_hashing_seq, eq_by_sorting_par, eq_by_sorting_seq, hash_par, hash_seq,
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

    c.bench_function("seq hashing", |b| {
        b.iter_batched(
            || data.clone(),
            |data| hash_seq(data),
            criterion::BatchSize::LargeInput,
        );
    });

    c.bench_function("par hashing", |b| {
        b.iter_batched(
            || data.clone(),
            |data| hash_par(data),
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

    c.bench_function("seq compare via hashing", |b| {
        b.iter_batched(
            || (data.clone(), shuffled.clone()),
            |(data, shuffled)| eq_by_hashing_seq(data, shuffled),
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

    c.bench_function("par compare via hashing", |b| {
        b.iter_batched(
            || (data.clone(), shuffled.clone()),
            |(data, shuffled)| eq_by_hashing_par(data, shuffled),
            BatchSize::LargeInput,
        );
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
