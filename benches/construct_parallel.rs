// benches/construct_parallel.rs

mod datasets;
mod support;

use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use int_interval::I32CO;
use int_interval_stack::IntCOStack;
use rayon::prelude::*;

use datasets::{Bounds, cases};

#[inline]
fn iv(start: i32, end_excl: i32) -> I32CO {
    I32CO::try_new(start, end_excl).unwrap()
}

#[inline]
fn intervals_from_bounds(bounds: &[Bounds]) -> Vec<I32CO> {
    bounds.iter().copied().map(|(s, e)| iv(s, e)).collect()
}

fn bench_construct_parallel(c: &mut Criterion) {
    let mut group = c.benchmark_group("construct_parallel");

    for &n in support::profile().sizes() {
        group.throughput(Throughput::Elements(n as u64));

        for (case, bounds) in cases(n) {
            let intervals = intervals_from_bounds(&bounds);
            let id = format!("{case}_{n}");

            group.bench_function(BenchmarkId::new("seq_collect", &id), |b| {
                b.iter(|| {
                    let stack: IntCOStack<I32CO> = black_box(&intervals).iter().copied().collect();

                    black_box(stack);
                });
            });

            group.bench_function(BenchmarkId::new("par_collect", &id), |b| {
                b.iter(|| {
                    let stack: IntCOStack<I32CO> =
                        black_box(&intervals).par_iter().copied().collect();

                    black_box(stack);
                });
            });
        }
    }

    group.finish();
}

criterion_group! {
    name = benches;
    config = support::config();
    targets = bench_construct_parallel
}

criterion_main!(benches);
