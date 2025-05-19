use minmax_element::{min_then_max, minmax_element, minmax_element_iter, minmax_element_unsafe};
use rand::seq::SliceRandom;

use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};

fn random_vec(size: usize) -> Vec<u64> {
    let mut v: Vec<_> = (0..size as u64).collect();
    let mut rng = rand::rng();
    v.shuffle(&mut rng);
    v
}

fn bench_minmax(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_minmax_element");
    let mut size = 16usize;
    while size <= 16 * 1024 * 1024 {
        group.bench_with_input(BenchmarkId::new("min_then_max", size), &size, |b, &size| {
            b.iter_batched(
                || random_vec(size),
                |v| min_then_max(&v),
                BatchSize::SmallInput,
            )
        });

        group.bench_with_input(
            BenchmarkId::new("minmax_element_iter", size),
            &size,
            |b, &size| {
                b.iter_batched(
                    || random_vec(size),
                    |v| minmax_element_iter(&v),
                    BatchSize::SmallInput,
                )
            },
        );

        group.bench_with_input(
            BenchmarkId::new("minmax_element", size),
            &size,
            |b, &size| {
                b.iter_batched(
                    || random_vec(size),
                    |v| minmax_element(&v),
                    BatchSize::SmallInput,
                )
            },
        );

        group.bench_with_input(
            BenchmarkId::new("minmax_element_unsafe", size),
            &size,
            |b, &size| {
                b.iter_batched(
                    || random_vec(size),
                    |v| minmax_element_unsafe(&v),
                    BatchSize::SmallInput,
                )
            },
        );
        size <<= 4;
    }
    group.finish();
}

criterion_group!(benches, bench_minmax);
criterion_main!(benches);
