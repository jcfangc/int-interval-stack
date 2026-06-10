// benches/height_at.rs

use divan::{Bencher, black_box};
use int_interval::I32CO;
use int_interval_stack::IntCOStack;

fn main() {
    divan::main();
}

type Bounds = (i32, i32);

#[inline]
fn iv(start: i32, end_excl: i32) -> I32CO {
    I32CO::try_new(start, end_excl).unwrap()
}

#[inline]
fn stack_from_bounds(bounds: &[Bounds]) -> IntCOStack<I32CO> {
    bounds.iter().copied().map(|(s, e)| iv(s, e)).collect()
}

fn sorted_disjoint(n: usize) -> Vec<Bounds> {
    (0..n)
        .map(|i| {
            let start = i as i32 * 4;
            (start, start + 2)
        })
        .collect()
}

fn reversed_disjoint(n: usize) -> Vec<Bounds> {
    let mut v = sorted_disjoint(n);
    v.reverse();
    v
}

fn adjacent_chain(n: usize) -> Vec<Bounds> {
    (0..n)
        .map(|i| {
            let start = i as i32 * 2;
            (start, start + 2)
        })
        .collect()
}

fn nested_dense(n: usize) -> Vec<Bounds> {
    (0..n)
        .map(|i| {
            let start = i as i32;
            let end_excl = (n * 2) as i32 - i as i32;
            (start, end_excl.max(start + 1))
        })
        .collect()
}

fn shifted_overlap(n: usize) -> Vec<Bounds> {
    (0..n)
        .map(|i| {
            let start = i as i32;
            (start, start + 32)
        })
        .collect()
}

fn mixed_unsorted(n: usize) -> Vec<Bounds> {
    let groups = n.div_ceil(4);
    let mut v = Vec::with_capacity(groups * 4);

    for i in 0..groups {
        let base = i as i32 * 40;
        v.extend([
            (base + 8, base + 18),
            (base, base + 10),
            (base + 24, base + 30),
            (base + 18, base + 24),
        ]);
    }

    v.reverse();
    v.truncate(n);
    v
}

fn point_queries(bounds: &[Bounds]) -> Vec<i32> {
    let mut out = Vec::with_capacity(bounds.len().min(128) * 3 + 2);

    for &(s, e) in bounds.iter().take(128) {
        out.push(s);
        out.push(s + (e - s) / 2);
        out.push(e);
    }

    if let Some(&(s, _)) = bounds.first() {
        out.push(s - 1);
    }
    if let Some(&(_, e)) = bounds.last() {
        out.push(e + 1);
    }

    out.sort_unstable();
    out.dedup();
    out
}

fn bench_height_at(bencher: Bencher, bounds: Vec<Bounds>) {
    let stack = stack_from_bounds(&bounds);
    let queries = point_queries(&bounds);

    bencher.bench(|| {
        let mut acc = 0usize;

        for &x in black_box(&queries) {
            acc = acc.wrapping_add(stack.height_at(black_box(x)));
        }

        black_box(acc)
    });
}

macro_rules! bench_height_at {
    ($fn_name:ident, $name:literal, $dataset:ident, $n:expr) => {
        #[divan::bench(name = $name)]
        fn $fn_name(bencher: Bencher) {
            bench_height_at(bencher, $dataset($n));
        }
    };
}

bench_height_at!(
    height_at_sorted_disjoint_64,
    "height_at/sorted_disjoint_64/int_interval_stack",
    sorted_disjoint,
    64
);

bench_height_at!(
    height_at_reversed_disjoint_64,
    "height_at/reversed_disjoint_64/int_interval_stack",
    reversed_disjoint,
    64
);

bench_height_at!(
    height_at_adjacent_chain_64,
    "height_at/adjacent_chain_64/int_interval_stack",
    adjacent_chain,
    64
);

bench_height_at!(
    height_at_nested_dense_64,
    "height_at/nested_dense_64/int_interval_stack",
    nested_dense,
    64
);

bench_height_at!(
    height_at_shifted_overlap_64,
    "height_at/shifted_overlap_64/int_interval_stack",
    shifted_overlap,
    64
);

bench_height_at!(
    height_at_mixed_unsorted_64,
    "height_at/mixed_unsorted_64/int_interval_stack",
    mixed_unsorted,
    64
);

bench_height_at!(
    height_at_sorted_disjoint_256,
    "height_at/sorted_disjoint_256/int_interval_stack",
    sorted_disjoint,
    256
);

bench_height_at!(
    height_at_reversed_disjoint_256,
    "height_at/reversed_disjoint_256/int_interval_stack",
    reversed_disjoint,
    256
);

bench_height_at!(
    height_at_adjacent_chain_256,
    "height_at/adjacent_chain_256/int_interval_stack",
    adjacent_chain,
    256
);

bench_height_at!(
    height_at_nested_dense_256,
    "height_at/nested_dense_256/int_interval_stack",
    nested_dense,
    256
);

bench_height_at!(
    height_at_shifted_overlap_256,
    "height_at/shifted_overlap_256/int_interval_stack",
    shifted_overlap,
    256
);

bench_height_at!(
    height_at_mixed_unsorted_256,
    "height_at/mixed_unsorted_256/int_interval_stack",
    mixed_unsorted,
    256
);

bench_height_at!(
    height_at_sorted_disjoint_1024,
    "height_at/sorted_disjoint_1024/int_interval_stack",
    sorted_disjoint,
    1024
);

bench_height_at!(
    height_at_reversed_disjoint_1024,
    "height_at/reversed_disjoint_1024/int_interval_stack",
    reversed_disjoint,
    1024
);

bench_height_at!(
    height_at_adjacent_chain_1024,
    "height_at/adjacent_chain_1024/int_interval_stack",
    adjacent_chain,
    1024
);

bench_height_at!(
    height_at_nested_dense_1024,
    "height_at/nested_dense_1024/int_interval_stack",
    nested_dense,
    1024
);

bench_height_at!(
    height_at_shifted_overlap_1024,
    "height_at/shifted_overlap_1024/int_interval_stack",
    shifted_overlap,
    1024
);

bench_height_at!(
    height_at_mixed_unsorted_1024,
    "height_at/mixed_unsorted_1024/int_interval_stack",
    mixed_unsorted,
    1024
);
