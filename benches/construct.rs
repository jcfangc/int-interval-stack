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

fn bench_stack_from_bounds(bencher: Bencher, bounds: Vec<Bounds>) {
    bencher.bench(|| {
        black_box(stack_from_bounds(black_box(&bounds)));
    });
}

macro_rules! bench_construct {
    ($fn_name:ident, $case:literal, $dataset:ident, $n:literal) => {
        #[divan::bench(name = concat!("construct/", $case, "_", stringify!($n), "/stack_from_bounds"))]
        fn $fn_name(bencher: Bencher) {
            bench_stack_from_bounds(bencher, $dataset($n));
        }
    };
}

bench_construct!(
    construct_sorted_disjoint_64,
    "sorted_disjoint",
    sorted_disjoint,
    64
);

bench_construct!(
    construct_reversed_disjoint_64,
    "reversed_disjoint",
    reversed_disjoint,
    64
);

bench_construct!(
    construct_adjacent_chain_64,
    "adjacent_chain",
    adjacent_chain,
    64
);

bench_construct!(construct_nested_dense_64, "nested_dense", nested_dense, 64);

bench_construct!(
    construct_shifted_overlap_64,
    "shifted_overlap",
    shifted_overlap,
    64
);

bench_construct!(
    construct_mixed_unsorted_64,
    "mixed_unsorted",
    mixed_unsorted,
    64
);

bench_construct!(
    construct_sorted_disjoint_256,
    "sorted_disjoint",
    sorted_disjoint,
    256
);

bench_construct!(
    construct_reversed_disjoint_256,
    "reversed_disjoint",
    reversed_disjoint,
    256
);

bench_construct!(
    construct_adjacent_chain_256,
    "adjacent_chain",
    adjacent_chain,
    256
);

bench_construct!(
    construct_nested_dense_256,
    "nested_dense",
    nested_dense,
    256
);

bench_construct!(
    construct_shifted_overlap_256,
    "shifted_overlap",
    shifted_overlap,
    256
);

bench_construct!(
    construct_mixed_unsorted_256,
    "mixed_unsorted",
    mixed_unsorted,
    256
);

bench_construct!(
    construct_sorted_disjoint_1024,
    "sorted_disjoint",
    sorted_disjoint,
    1024
);

bench_construct!(
    construct_reversed_disjoint_1024,
    "reversed_disjoint",
    reversed_disjoint,
    1024
);

bench_construct!(
    construct_adjacent_chain_1024,
    "adjacent_chain",
    adjacent_chain,
    1024
);

bench_construct!(
    construct_nested_dense_1024,
    "nested_dense",
    nested_dense,
    1024
);

bench_construct!(
    construct_shifted_overlap_1024,
    "shifted_overlap",
    shifted_overlap,
    1024
);

bench_construct!(
    construct_mixed_unsorted_1024,
    "mixed_unsorted",
    mixed_unsorted,
    1024
);
