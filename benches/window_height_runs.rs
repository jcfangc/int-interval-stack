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

fn dense_heights(stack: &IntCOStack<I32CO>, from: i32, to: i32) -> Vec<usize> {
    (from..to).map(|x| stack.height_at(x)).collect()
}

fn checksum_stack_window_runs(stack: &IntCOStack<I32CO>, from: i32, to: i32, len: u32) -> i64 {
    let mut acc = 0_i64;

    for window in stack.iter_windows(from, to, len) {
        for run in window.iter_height_runs() {
            acc ^= (run.interval.start() as i64) << 1;
            acc ^= (run.interval.end_excl() as i64) << 2;
            acc ^= run.height as i64;
        }
    }

    acc
}

fn checksum_dense_window_runs(dense: &[usize], from: i32, len: usize) -> i64 {
    if len == 0 {
        return 0;
    }

    let mut acc = 0_i64;

    for (window_start, window) in dense.windows(len).enumerate() {
        let base = from as i64 + window_start as i64;

        let mut run_start = 0_usize;
        let mut height = window[0];

        for (i, &next_height) in window.iter().enumerate().skip(1) {
            if next_height == height {
                continue;
            }

            acc ^= (base + run_start as i64) << 1;
            acc ^= (base + i as i64) << 2;
            acc ^= height as i64;

            run_start = i;
            height = next_height;
        }

        acc ^= (base + run_start as i64) << 1;
        acc ^= (base + window.len() as i64) << 2;
        acc ^= height as i64;
    }

    acc
}

fn bench_stack_window_runs(bencher: Bencher, bounds: Vec<Bounds>, len: u32) {
    let (from, to) = domain_from_bounds(&bounds).unwrap();
    let stack = stack_from_bounds(&bounds);

    bencher.bench(|| {
        black_box(checksum_stack_window_runs(
            black_box(&stack),
            black_box(from),
            black_box(to),
            black_box(len),
        ))
    });
}

fn bench_dense_query_only(bencher: Bencher, bounds: Vec<Bounds>, len: u32) {
    let (from, to) = domain_from_bounds(&bounds).unwrap();
    let stack = stack_from_bounds(&bounds);
    let dense = dense_heights(&stack, from, to);

    bencher.bench(|| {
        black_box(checksum_dense_window_runs(
            black_box(&dense),
            black_box(from),
            black_box(len as usize),
        ))
    });
}

fn bench_dense_end_to_end(bencher: Bencher, bounds: Vec<Bounds>, len: u32) {
    let (from, to) = domain_from_bounds(&bounds).unwrap();
    let stack = stack_from_bounds(&bounds);

    bencher.bench(|| {
        let dense = dense_heights(black_box(&stack), black_box(from), black_box(to));

        black_box(checksum_dense_window_runs(
            black_box(&dense),
            black_box(from),
            black_box(len as usize),
        ))
    });
}

macro_rules! bench_case {
    ($mod_name:ident, $case:literal, $dataset:ident, $n:literal, $len:literal) => {
        mod $mod_name {
            use super::*;

            #[divan::bench(name = concat!(
                                                        "window_height_runs/",
                                                        $case,
                                                        "_",
                                                        stringify!($n),
                                                        "_len_",
                                                        stringify!($len),
                                                        "/int_interval_stack"
                                                    ))]
            fn int_interval_stack(bencher: Bencher) {
                bench_stack_window_runs(bencher, $dataset($n), $len);
            }

            #[divan::bench(name = concat!(
                                                        "window_height_runs/",
                                                        $case,
                                                        "_",
                                                        stringify!($n),
                                                        "_len_",
                                                        stringify!($len),
                                                        "/std_dense_query_only"
                                                    ))]
            fn std_dense_query_only(bencher: Bencher) {
                bench_dense_query_only(bencher, $dataset($n), $len);
            }

            #[divan::bench(name = concat!(
                                                        "window_height_runs/",
                                                        $case,
                                                        "_",
                                                        stringify!($n),
                                                        "_len_",
                                                        stringify!($len),
                                                        "/std_dense_end_to_end"
                                                    ))]
            fn std_dense_end_to_end(bencher: Bencher) {
                bench_dense_end_to_end(bencher, $dataset($n), $len);
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
