// benches/peak_intervals.rs

mod datasets;
mod support;

use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use datasets::{cases, stack_from_bounds};

fn bench_peak_intervals(c: &mut Criterion) {
    let mut group = c.benchmark_group("peak_intervals");

    for &n in support::profile().sizes() {
        for (case, bounds) in cases(n) {
            let stack = stack_from_bounds(&bounds);
            let point_count = stack.change_points().len().max(1);
            let peak_count = stack.peak_intervals().count();
            let id = format!("{case}_{n}_peaks_{peak_count}");

            group.throughput(Throughput::Elements(point_count as u64));

            group.bench_function(BenchmarkId::new("int_interval_stack", id), |b| {
                b.iter(|| {
                    let mut acc = 0i64;

                    for (iv, h) in stack.peak_intervals() {
                        acc ^= (iv.start() as i64) << 1;
                        acc ^= (iv.end_excl() as i64) << 2;
                        acc ^= h as i64;
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
    targets = bench_peak_intervals
}

criterion_main!(benches);
