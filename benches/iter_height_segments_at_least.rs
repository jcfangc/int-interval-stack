// benches/iter_height_segments_at_least.rs

mod datasets;
mod support;

use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use datasets::{cases, stack_from_bounds};

fn thresholds(max_height: usize) -> Vec<usize> {
    let mut values = vec![1, 2, max_height / 2, max_height];
    values.retain(|&x| x != 0);
    values.sort_unstable();
    values.dedup();
    values
}

fn bench_iter_height_segments_at_least(c: &mut Criterion) {
    let mut group = c.benchmark_group("iter_height_segments_at_least");

    for &n in support::profile().sizes() {
        for (case, bounds) in cases(n) {
            let stack = stack_from_bounds(&bounds);
            let point_count = stack.change_points().len().max(1);
            let max_height = stack.height_stats().max_height();
            let id = format!("{case}_{n}");

            group.throughput(Throughput::Elements(point_count as u64));

            for min_height in thresholds(max_height) {
                group.bench_function(
                    BenchmarkId::new(format!("int_interval_stack_min_{min_height}"), &id),
                    |b| {
                        b.iter(|| {
                            let mut acc = 0i64;

                            for height_seg in
                                stack.iter_height_segments_at_least(black_box(min_height))
                            {
                                acc ^= (height_seg.interval.start() as i64) << 1;
                                acc ^= (height_seg.interval.end_excl() as i64) << 2;
                                acc ^= height_seg.height as i64;
                            }

                            black_box(acc);
                        });
                    },
                );
            }
        }
    }

    group.finish();
}

criterion_group! {
    name = benches;
    config = support::config();
    targets = bench_iter_height_segments_at_least
}

criterion_main!(benches);
