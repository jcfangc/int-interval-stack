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

fn nested_dense(n: usize) -> Vec<Bounds> {
    (0..n)
        .map(|i| {
            let start = i as i32;
            let end_excl = (n * 2) as i32 - i as i32;
            (start, end_excl.max(start + 1))
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

fn domain_from_bounds(bounds: &[Bounds]) -> Option<(i32, i32)> {
    let from = bounds.iter().map(|&(s, _)| s).min()?;
    let to = bounds.iter().map(|&(_, e)| e).max()?;

    (from < to).then_some((from, to))
}

fn checksum_window_bounds(stack: &IntCOStack<I32CO>, from: i32, to: i32, len: u32) -> i64 {
    let mut acc = 0_i64;

    for window in stack.iter_windows(from, to, len) {
        let interval = window.interval();

        acc ^= (interval.start() as i64) << 1;
        acc ^= (interval.end_excl() as i64) << 2;
    }

    acc
}

fn bench_window_iter_bounds(bencher: Bencher, bounds: Vec<Bounds>, len: u32) {
    let (from, to) = domain_from_bounds(&bounds).unwrap();
    let stack = stack_from_bounds(&bounds);

    bencher.bench(|| {
        black_box(checksum_window_bounds(
            black_box(&stack),
            black_box(from),
            black_box(to),
            black_box(len),
        ))
    });
}

macro_rules! bench_case {
    ($mod_name:ident, $case:literal, $dataset:ident, $n:literal, $len:literal) => {
        mod $mod_name {
            use super::*;

            #[divan::bench(name = concat!(
                                                        "window_iter_bounds/n_",
                                                        stringify!($n),
                                                        "_len_",
                                                        stringify!($len),
                                                        "/",
                                                        $case
                                                    ))]
            fn run(bencher: Bencher) {
                bench_window_iter_bounds(bencher, $dataset($n), $len);
            }
        }
    };
}

bench_case!(
    sorted_disjoint_64_len_8,
    "sorted_disjoint",
    sorted_disjoint,
    64,
    8
);
bench_case!(
    sorted_disjoint_64_len_128,
    "sorted_disjoint",
    sorted_disjoint,
    64,
    128
);
bench_case!(nested_dense_64_len_8, "nested_dense", nested_dense, 64, 8);
bench_case!(
    nested_dense_64_len_128,
    "nested_dense",
    nested_dense,
    64,
    128
);
bench_case!(
    mixed_unsorted_64_len_8,
    "mixed_unsorted",
    mixed_unsorted,
    64,
    8
);
bench_case!(
    mixed_unsorted_64_len_128,
    "mixed_unsorted",
    mixed_unsorted,
    64,
    128
);

bench_case!(
    sorted_disjoint_256_len_8,
    "sorted_disjoint",
    sorted_disjoint,
    256,
    8
);
bench_case!(
    sorted_disjoint_256_len_128,
    "sorted_disjoint",
    sorted_disjoint,
    256,
    128
);
bench_case!(nested_dense_256_len_8, "nested_dense", nested_dense, 256, 8);
bench_case!(
    nested_dense_256_len_128,
    "nested_dense",
    nested_dense,
    256,
    128
);
bench_case!(
    mixed_unsorted_256_len_8,
    "mixed_unsorted",
    mixed_unsorted,
    256,
    8
);
bench_case!(
    mixed_unsorted_256_len_128,
    "mixed_unsorted",
    mixed_unsorted,
    256,
    128
);

bench_case!(
    sorted_disjoint_1024_len_8,
    "sorted_disjoint",
    sorted_disjoint,
    1024,
    8
);
bench_case!(
    sorted_disjoint_1024_len_128,
    "sorted_disjoint",
    sorted_disjoint,
    1024,
    128
);
bench_case!(
    nested_dense_1024_len_8,
    "nested_dense",
    nested_dense,
    1024,
    8
);
bench_case!(
    nested_dense_1024_len_128,
    "nested_dense",
    nested_dense,
    1024,
    128
);
bench_case!(
    mixed_unsorted_1024_len_8,
    "mixed_unsorted",
    mixed_unsorted,
    1024,
    8
);
bench_case!(
    mixed_unsorted_1024_len_128,
    "mixed_unsorted",
    mixed_unsorted,
    1024,
    128
);
