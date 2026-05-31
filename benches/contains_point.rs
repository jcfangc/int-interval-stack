// benches/contains_point.rs

mod datasets;
mod support;

use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use datasets::{cases, point_queries, stack_from_bounds};

fn bench_contains_point(c: &mut Criterion) {
    let mut group = c.benchmark_group("contains_point");

    for &n in support::profile().sizes() {
        for (case, bounds) in cases(n) {
            let stack = stack_from_bounds(&bounds);
            let queries = point_queries(&bounds);
            let id = format!("{case}_{n}");

            group.throughput(Throughput::Elements(queries.len() as u64));

            group.bench_function(BenchmarkId::new("int_interval_stack", &id), |b| {
                b.iter(|| {
                    let mut acc = 0usize;

                    for &x in black_box(&queries) {
                        acc ^= stack.contains_point(black_box(x)) as usize;
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
    targets = bench_contains_point
}

criterion_main!(benches);
