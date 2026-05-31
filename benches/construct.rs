mod datasets;
mod support;

use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use datasets::{cases, stack_from_bounds};

fn bench_construct(c: &mut Criterion) {
    let mut group = c.benchmark_group("construct");

    for &n in support::profile().sizes() {
        group.throughput(Throughput::Elements(n as u64));

        for (case, bounds) in cases(n) {
            group.bench_function(
                BenchmarkId::new("int_interval_stack", format!("{case}_{n}")),
                |b| {
                    b.iter(|| black_box(stack_from_bounds(black_box(&bounds))));
                },
            );
        }
    }

    group.finish();
}

criterion_group! {
    name = benches;
    config = support::config();
    targets = bench_construct
}

criterion_main!(benches);
