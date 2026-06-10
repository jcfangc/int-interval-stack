// benches/construct_parallel.rs

use divan::{Bencher, black_box};
use int_interval::I32CO;
use int_interval_stack::IntCOStack;
use rayon::prelude::*;

fn main() {
    divan::main();
}

type Bounds = (i32, i32);

const N64: usize = 64;
const N256: usize = 256;
const N1024: usize = 1024;

#[inline]
fn iv(start: i32, end_excl: i32) -> I32CO {
    I32CO::try_new(start, end_excl).unwrap()
}

#[inline]
fn intervals_from_bounds(bounds: &[Bounds]) -> Vec<I32CO> {
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

fn bench_seq_collect(bencher: Bencher, bounds: Vec<Bounds>) {
    let intervals = intervals_from_bounds(&bounds);

    bencher.bench(|| {
        let stack: IntCOStack<I32CO> = black_box(&intervals).iter().copied().collect();
        black_box(stack);
    });
}

fn bench_par_collect(bencher: Bencher, bounds: Vec<Bounds>) {
    let intervals = intervals_from_bounds(&bounds);

    bencher.bench(|| {
        let stack: IntCOStack<I32CO> = black_box(&intervals).par_iter().copied().collect();
        black_box(stack);
    });
}

macro_rules! bench_construct_parallel {
    ($seq_fn:ident, $par_fn:ident, $case:literal, $dataset:ident, $n:expr) => {
        #[divan::bench(name = concat!("construct_parallel/", $case, "_", stringify!($n), "/seq_collect"))]
        fn $seq_fn(bencher: Bencher) {
            bench_seq_collect(bencher, $dataset($n));
        }

        #[divan::bench(name = concat!("construct_parallel/", $case, "_", stringify!($n), "/par_collect"))]
        fn $par_fn(bencher: Bencher) {
            bench_par_collect(bencher, $dataset($n));
        }
    };
}

bench_construct_parallel!(
    construct_parallel_sorted_disjoint_64_seq,
    construct_parallel_sorted_disjoint_64_par,
    "sorted_disjoint",
    sorted_disjoint,
    N64
);

bench_construct_parallel!(
    construct_parallel_reversed_disjoint_64_seq,
    construct_parallel_reversed_disjoint_64_par,
    "reversed_disjoint",
    reversed_disjoint,
    N64
);

bench_construct_parallel!(
    construct_parallel_adjacent_chain_64_seq,
    construct_parallel_adjacent_chain_64_par,
    "adjacent_chain",
    adjacent_chain,
    N64
);

bench_construct_parallel!(
    construct_parallel_nested_dense_64_seq,
    construct_parallel_nested_dense_64_par,
    "nested_dense",
    nested_dense,
    N64
);

bench_construct_parallel!(
    construct_parallel_shifted_overlap_64_seq,
    construct_parallel_shifted_overlap_64_par,
    "shifted_overlap",
    shifted_overlap,
    N64
);

bench_construct_parallel!(
    construct_parallel_mixed_unsorted_64_seq,
    construct_parallel_mixed_unsorted_64_par,
    "mixed_unsorted",
    mixed_unsorted,
    N64
);

bench_construct_parallel!(
    construct_parallel_sorted_disjoint_256_seq,
    construct_parallel_sorted_disjoint_256_par,
    "sorted_disjoint",
    sorted_disjoint,
    N256
);

bench_construct_parallel!(
    construct_parallel_reversed_disjoint_256_seq,
    construct_parallel_reversed_disjoint_256_par,
    "reversed_disjoint",
    reversed_disjoint,
    N256
);

bench_construct_parallel!(
    construct_parallel_adjacent_chain_256_seq,
    construct_parallel_adjacent_chain_256_par,
    "adjacent_chain",
    adjacent_chain,
    N256
);

bench_construct_parallel!(
    construct_parallel_nested_dense_256_seq,
    construct_parallel_nested_dense_256_par,
    "nested_dense",
    nested_dense,
    N256
);

bench_construct_parallel!(
    construct_parallel_shifted_overlap_256_seq,
    construct_parallel_shifted_overlap_256_par,
    "shifted_overlap",
    shifted_overlap,
    N256
);

bench_construct_parallel!(
    construct_parallel_mixed_unsorted_256_seq,
    construct_parallel_mixed_unsorted_256_par,
    "mixed_unsorted",
    mixed_unsorted,
    N256
);

bench_construct_parallel!(
    construct_parallel_sorted_disjoint_1024_seq,
    construct_parallel_sorted_disjoint_1024_par,
    "sorted_disjoint",
    sorted_disjoint,
    N1024
);

bench_construct_parallel!(
    construct_parallel_reversed_disjoint_1024_seq,
    construct_parallel_reversed_disjoint_1024_par,
    "reversed_disjoint",
    reversed_disjoint,
    N1024
);

bench_construct_parallel!(
    construct_parallel_adjacent_chain_1024_seq,
    construct_parallel_adjacent_chain_1024_par,
    "adjacent_chain",
    adjacent_chain,
    N1024
);

bench_construct_parallel!(
    construct_parallel_nested_dense_1024_seq,
    construct_parallel_nested_dense_1024_par,
    "nested_dense",
    nested_dense,
    N1024
);

bench_construct_parallel!(
    construct_parallel_shifted_overlap_1024_seq,
    construct_parallel_shifted_overlap_1024_par,
    "shifted_overlap",
    shifted_overlap,
    N1024
);

bench_construct_parallel!(
    construct_parallel_mixed_unsorted_1024_seq,
    construct_parallel_mixed_unsorted_1024_par,
    "mixed_unsorted",
    mixed_unsorted,
    N1024
);
