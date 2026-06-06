use int_interval::I32CO;
use int_interval_stack::IntCOStack;
use rayon::iter::ParallelIterator;

use crate::datasets::Bounds;

pub(crate) fn domain_from_bounds(bounds: &[Bounds]) -> Option<(i32, i32)> {
    let from = bounds.iter().map(|&(s, _)| s).min()?;
    let to = bounds.iter().map(|&(_, e)| e).max()?;

    (from < to).then_some((from, to))
}

pub(crate) fn window_lens(from: i32, to: i32) -> Vec<u32> {
    let span: u32 = (to - from)
        .try_into()
        .expect("benchmark domains must fit u32");

    let mut lens = vec![8, 32, span.div_ceil(4)];
    lens.retain(|&len| len != 0 && len <= span);
    lens.sort_unstable();
    lens.dedup();

    if lens.is_empty() {
        lens.push(1);
    }

    lens
}

pub(crate) fn window_count(from: i32, to: i32, len: u32) -> usize {
    if from >= to || len == 0 {
        return 0;
    }

    let span: u32 = (to - from)
        .try_into()
        .expect("benchmark domains must fit u32");

    if len > span {
        return 0;
    }

    (span - len + 1) as usize
}

#[allow(dead_code)]
pub(crate) fn dense_heights(stack: &IntCOStack<I32CO>, from: i32, to: i32) -> Vec<usize> {
    (from..to).map(|x| stack.height_at(x)).collect()
}

#[allow(dead_code)]
pub(crate) fn checksum_stack_window_bounds(
    stack: &IntCOStack<I32CO>,
    from: i32,
    to: i32,
    len: u32,
) -> i64 {
    let mut acc = 0i64;

    for window in stack.iter_windows(from, to, len) {
        let interval = window.interval();

        acc ^= (interval.start() as i64) << 1;
        acc ^= (interval.end_excl() as i64) << 2;
    }

    acc
}

#[allow(dead_code)]
pub(crate) fn checksum_stack_window_bounds_par(
    stack: &IntCOStack<I32CO>,
    from: i32,
    to: i32,
    len: u32,
) -> i64 {
    stack
        .par_iter_windows(from, to, len)
        .map(|window| {
            let interval = window.interval();

            ((interval.start() as i64) << 1) ^ ((interval.end_excl() as i64) << 2)
        })
        .reduce(|| 0, |a, b| a ^ b)
}

#[allow(dead_code)]
pub(crate) fn checksum_stack_window_runs(
    stack: &IntCOStack<I32CO>,
    from: i32,
    to: i32,
    len: u32,
) -> i64 {
    let mut acc = 0i64;

    for window in stack.iter_windows(from, to, len) {
        for run in window.iter_height_runs() {
            acc ^= (run.interval.start() as i64) << 1;
            acc ^= (run.interval.end_excl() as i64) << 2;
            acc ^= run.height as i64;
        }
    }

    acc
}

#[allow(dead_code)]
pub(crate) fn checksum_stack_window_runs_par(
    stack: &IntCOStack<I32CO>,
    from: i32,
    to: i32,
    len: u32,
) -> i64 {
    let mut acc = 0i64;

    for window in stack.iter_windows(from, to, len) {
        let window_acc = window
            .par_iter_height_runs()
            .map(|run| {
                ((run.interval.start() as i64) << 1)
                    ^ ((run.interval.end_excl() as i64) << 2)
                    ^ run.height as i64
            })
            .reduce(|| 0, |a, b| a ^ b);

        acc ^= window_acc;
    }

    acc
}

#[allow(dead_code)]
pub(crate) fn checksum_dense_window_runs(dense: &[usize], from: i32, len: usize) -> i64 {
    if len == 0 {
        return 0;
    }

    let mut acc = 0i64;

    for (window_start, window) in dense.windows(len).enumerate() {
        let base = from as i64 + window_start as i64;

        let mut run_start = 0usize;
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
