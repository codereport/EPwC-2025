use min12::{less, min_element12, min_element12_practical};
use rand::seq::SliceRandom;

use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};

fn random_vec(size: usize) -> Vec<u64> {
    let mut v: Vec<_> = (0..size as u64).collect();
    let mut rng = rand::rng();
    v.shuffle(&mut rng);
    v
}

fn bench_min12(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_min12");
    let mut size = 16usize;
    while size <= 16 * 1024 * 1024 {
        group.bench_with_input(
            BenchmarkId::new("min_element12", size),
            &size,
            |b, &size| {
                b.iter_batched(
                    || random_vec(size),
                    |v| min_element12(&v, 0, v.len(), less()),
                    BatchSize::SmallInput,
                )
            },
        );

        group.bench_with_input(
            BenchmarkId::new("min_element12_practical", size),
            &size,
            |b, &size| {
                b.iter_batched(
                    || random_vec(size),
                    |v| min_element12_practical(&v, 0, v.len(), less()),
                    BatchSize::SmallInput,
                )
            },
        );
        size <<= 4;
    }
    group.finish();
}

criterion_group!(benches, bench_min12);
criterion_main!(benches);
