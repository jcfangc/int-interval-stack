use rayon::iter::ParallelIterator;

use crate::int_co_stack::test_support::{iv, stack_from_intervals};

use super::*;

fn collect_runs(window: StackWindow<'_, int_interval::I32CO>) -> Vec<((i32, i32), usize)> {
    window
        .iter_height_runs()
        .map(|run| ((run.interval.start(), run.interval.end_excl()), run.height))
        .collect()
}

fn collect_runs_parallel(window: StackWindow<'_, int_interval::I32CO>) -> Vec<((i32, i32), usize)> {
    window
        .par_iter_height_runs()
        .map(|run| ((run.interval.start(), run.interval.end_excl()), run.height))
        .collect()
}

#[test]
fn serial_and_parallel_height_runs_match_for_single_run() {
    let stack = stack_from_intervals(&[(2, 8)]);
    let window = StackWindow::new(&stack, iv(3, 6));

    assert_eq!(collect_runs(window), collect_runs_parallel(window));
}

#[test]
fn serial_and_parallel_height_runs_match_for_multiple_runs() {
    let stack = stack_from_intervals(&[(1, 4), (3, 6)]);
    let window = StackWindow::new(&stack, iv(2, 5));

    assert_eq!(
        collect_runs(window),
        vec![((2, 3), 1), ((3, 4), 2), ((4, 5), 1)],
    );

    assert_eq!(collect_runs(window), collect_runs_parallel(window));
}

#[test]
fn serial_and_parallel_height_runs_match_with_zero_gap() {
    let stack = stack_from_intervals(&[(1, 3), (5, 7)]);
    let window = StackWindow::new(&stack, iv(2, 6));

    assert_eq!(
        collect_runs(window),
        vec![((2, 3), 1), ((3, 5), 0), ((5, 6), 1)],
    );

    assert_eq!(collect_runs(window), collect_runs_parallel(window));
}

#[test]
fn serial_and_parallel_height_runs_match_after_all_change_points() {
    let stack = stack_from_intervals(&[(1, 3)]);
    let window = StackWindow::new(&stack, iv(3, 6));

    assert_eq!(collect_runs(window), vec![((3, 6), 0)]);
    assert_eq!(collect_runs(window), collect_runs_parallel(window));
}

#[test]
fn serial_and_parallel_height_runs_match_when_window_edges_are_change_points() {
    let stack = stack_from_intervals(&[(1, 4), (3, 6)]);
    let window = StackWindow::new(&stack, iv(1, 6));

    assert_eq!(
        collect_runs(window),
        vec![((1, 3), 1), ((3, 4), 2), ((4, 6), 1)],
    );

    assert_eq!(collect_runs(window), collect_runs_parallel(window));
}
