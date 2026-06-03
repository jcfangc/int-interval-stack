// benches/iter_height_segments_exactly.rs

mod datasets;
mod support;

use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};

use datasets::{cases, stack_from_bounds};

fn target_heights(max_height: usize) -> Vec<usize> {
    let mut targets = vec![1];

    if max_height > 1 {
        targets.push(max_height.div_ceil(2));
        targets.push(max_height);
    }

    targets.sort_unstable();
    targets.dedup();
    targets
}

fn bench_iter_height_segments_exactly(c: &mut Criterion) {
    let mut group = c.benchmark_group("iter_height_segments_exactly");

    for &n in support::profile().sizes() {
        for (case, bounds) in cases(n) {
            let stack = stack_from_bounds(&bounds);
            let point_count = stack.change_points().len().max(1);
            let max_height = stack.height_stats().max_height();
            let id = format!("{case}_{n}");

            group.throughput(Throughput::Elements(point_count as u64));

            for target in target_heights(max_height) {
                group.bench_function(BenchmarkId::new(format!("height_{target}"), &id), |b| {
                    b.iter(|| {
                        let stack = black_box(&stack);
                        let target = black_box(target);
                        let mut acc = 0i64;

                        for (iv, h) in stack.iter_height_segments_exactly(target) {
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
    targets = bench_iter_height_segments_exactly
}

criterion_main!(benches);
