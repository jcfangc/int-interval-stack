// benches/contains_interval.rs

mod datasets;
mod support;

use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use datasets::{cases, interval_queries, iv, stack_from_bounds};

fn bench_contains_interval(c: &mut Criterion) {
    let mut group = c.benchmark_group("contains_interval");

    for &n in support::profile().sizes() {
        for (case, bounds) in cases(n) {
            let stack = stack_from_bounds(&bounds);
            let queries = interval_queries(&bounds);
            let id = format!("{case}_{n}");

            group.throughput(Throughput::Elements(queries.len() as u64));

            group.bench_function(BenchmarkId::new("int_interval_stack", &id), |b| {
                b.iter(|| {
                    let stack = black_box(&stack);
                    let queries = black_box(&queries);

                    let mut acc = 0usize;

                    for &(start, end_excl) in queries {
                        acc ^= stack.contains_interval(black_box(iv(start, end_excl))) as usize;
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
    targets = bench_contains_interval
}

criterion_main!(benches);
