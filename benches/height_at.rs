mod datasets;
mod support;

use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use datasets::{cases, point_queries, stack_from_bounds};

fn bench_height_at(c: &mut Criterion) {
    let mut group = c.benchmark_group("height_at");

    for &n in support::profile().sizes() {
        for (case, bounds) in cases(n) {
            let stack = stack_from_bounds(&bounds);
            let queries = point_queries(&bounds);

            group.throughput(Throughput::Elements(queries.len() as u64));

            group.bench_function(
                BenchmarkId::new("int_interval_stack", format!("{case}_{n}")),
                |b| {
                    b.iter(|| {
                        let mut acc = 0usize;
                        for &x in black_box(&queries) {
                            acc = acc.wrapping_add(stack.height_at(black_box(x)));
                        }
                        black_box(acc)
                    });
                },
            );
        }
    }

    group.finish();
}

criterion_group! {
    name = benches;
    config = support::config();
    targets = bench_height_at
}

criterion_main!(benches);
