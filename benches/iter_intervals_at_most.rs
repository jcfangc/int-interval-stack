// benches/iter_intervals_at_most.rs

mod datasets;
mod support;

use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use datasets::{cases, stack_from_bounds};

fn thresholds(max_height: usize) -> Vec<usize> {
    let mut out = vec![
        0,
        1,
        max_height / 2,
        max_height,
        max_height.saturating_add(1),
    ];
    out.sort_unstable();
    out.dedup();
    out
}

fn bench_iter_intervals_at_most(c: &mut Criterion) {
    let mut group = c.benchmark_group("iter_intervals_at_most");

    for &n in support::profile().sizes() {
        for (case, bounds) in cases(n) {
            let stack = stack_from_bounds(&bounds);
            let point_count = stack.change_points().len().max(1);
            let max_height = stack.max_height();

            group.throughput(Throughput::Elements(point_count as u64));

            for max in thresholds(max_height) {
                let id = format!("{case}_{n}_max_{max}");

                group.bench_function(BenchmarkId::new("int_interval_stack", id), |b| {
                    b.iter(|| {
                        let mut acc = 0i64;

                        for (iv, h) in stack.iter_height_segments_at_most(black_box(max)) {
                            acc ^= (iv.start() as i64) << 1;
                            acc ^= (iv.end_excl() as i64) << 2;
                            acc ^= h as i64;
                        }

                        black_box(acc);
                    });
                });
            }
        }
    }

    group.finish();
}

criterion_group! {
    name = benches;
    config = support::config();
    targets = bench_iter_intervals_at_most
}

criterion_main!(benches);
