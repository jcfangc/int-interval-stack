mod datasets;
mod support;
mod window_support;

use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use datasets::{cases, stack_from_bounds};
use window_support::{
    checksum_stack_window_bounds_par, domain_from_bounds, window_count, window_lens,
};

fn bench_par_iter_windows(c: &mut Criterion) {
    let mut group = c.benchmark_group("par_iter_windows");

    for &n in support::profile().sizes() {
        for (case, bounds) in cases(n) {
            let Some((from, to)) = domain_from_bounds(&bounds) else {
                continue;
            };

            let stack = stack_from_bounds(&bounds);

            for len in window_lens(from, to) {
                let windows = window_count(from, to, len).max(1);
                let id = format!("{case}_{n}_len_{len}");

                group.throughput(Throughput::Elements(windows as u64));

                group.bench_function(BenchmarkId::new("int_interval_stack", id), |b| {
                    b.iter(|| {
                        black_box(checksum_stack_window_bounds_par(
                            black_box(&stack),
                            black_box(from),
                            black_box(to),
                            black_box(len),
                        ))
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
    targets = bench_par_iter_windows
}

criterion_main!(benches);
