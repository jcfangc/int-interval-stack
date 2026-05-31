// benches/max_height.rs

mod datasets;
mod support;

use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use datasets::{cases, stack_from_bounds};

fn bench_max_height(c: &mut Criterion) {
    let mut group = c.benchmark_group("max_height");

    for &n in support::profile().sizes() {
        for (case, bounds) in cases(n) {
            let stack = stack_from_bounds(&bounds);
            let point_count = stack.change_points().len().max(1);
            let id = format!("{case}_{n}");

            group.throughput(Throughput::Elements(point_count as u64));

            group.bench_function(BenchmarkId::new("int_interval_stack", &id), |b| {
                b.iter(|| {
                    let stack = black_box(&stack);
                    black_box(stack.max_height());
                });
            });
        }
    }

    group.finish();
}

criterion_group! {
    name = benches;
    config = support::config();
    targets = bench_max_height
}

criterion_main!(benches);
