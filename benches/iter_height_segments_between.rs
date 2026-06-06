// benches/iter_height_segments_between.rs

mod datasets;
mod support;

use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};

use datasets::{cases, stack_from_bounds};

fn height_ranges(max_height: usize) -> Vec<(String, usize, usize)> {
    let max = max_height.max(1);
    let mid = max.div_ceil(2);

    let candidates = [
        ("all", 1, max),
        ("low", 1, mid),
        ("high", mid, max),
        ("peak", max, max),
    ];

    let mut out = Vec::new();

    for (name, min, max) in candidates {
        if !out.iter().any(|(_, l, r)| *l == min && *r == max) {
            out.push((format!("{name}_{min}_{max}"), min, max));
        }
    }

    out
}

fn bench_iter_height_segments_between(c: &mut Criterion) {
    let mut group = c.benchmark_group("iter_height_segments_between");

    for &n in support::profile().sizes() {
        for (case, bounds) in cases(n) {
            let stack = stack_from_bounds(&bounds);
            let point_count = stack.change_points().len().max(1);
            let max_height = stack.height_stats().max_height();
            let id = format!("{case}_{n}");

            group.throughput(Throughput::Elements(point_count as u64));

            for (range_name, min_height, max_height) in height_ranges(max_height) {
                group.bench_function(BenchmarkId::new(range_name, &id), |b| {
                    b.iter(|| {
                        let stack = black_box(&stack);
                        let min_height = black_box(min_height);
                        let max_height = black_box(max_height);

                        let mut acc = 0i64;

                        for height_seg in stack.iter_height_segments_between(min_height, max_height)
                        {
                            acc ^= (height_seg.interval.start() as i64) << 1;
                            acc ^= (height_seg.interval.end_excl() as i64) << 2;
                            acc ^= height_seg.height.get() as i64;
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
    targets = bench_iter_height_segments_between
}

criterion_main!(benches);
