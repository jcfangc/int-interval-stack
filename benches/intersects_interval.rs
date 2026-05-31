// benches/intersects_interval.rs

mod datasets;
mod support;

use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use datasets::{cases, interval_queries, stack_from_bounds};

fn bench_intersects_interval(c: &mut Criterion) {
    let mut group = c.benchmark_group("intersects_interval");

    for &n in support::profile().sizes() {
        for (case, bounds) in cases(n) {
            let stack = stack_from_bounds(&bounds);
            let queries = interval_queries(&bounds);
            let id = format!("{case}_{n}");

            group.throughput(Throughput::Elements(queries.len() as u64));

            group.bench_function(BenchmarkId::new("int_interval_stack", &id), |b| {
                b.iter(|| {
                    let mut acc = false;

                    for &(start, end_excl) in black_box(&queries) {
                        acc ^= stack.intersects_interval(black_box(datasets::iv(start, end_excl)));
                    }

                    black_box(acc);
                });
            });
        }
    }

    group.finish();
}

criterion_group! {
    name = benches;
    config = support::config();
    targets = bench_intersects_interval
}

criterion_main!(benches);
