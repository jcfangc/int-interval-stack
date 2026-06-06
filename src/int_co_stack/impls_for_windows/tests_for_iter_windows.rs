use int_interval::I8CO;
use rayon::iter::ParallelIterator;

use crate::int_co_stack::impls_for_windows::test_support::{
    run_bounds, stack_from_i8_intervals, window_bounds,
};

use super::*;

fn serial_windows(stack: &IntCOStack<I8CO>, from: i8, to: i8, len: u8) -> Vec<(i8, i8)> {
    stack
        .iter_windows(from, to, len)
        .map(window_bounds)
        .collect()
}

fn parallel_windows(stack: &IntCOStack<I8CO>, from: i8, to: i8, len: u8) -> Vec<(i8, i8)> {
    stack
        .par_iter_windows(from, to, len)
        .map(window_bounds)
        .collect()
}

#[test]
fn serial_and_parallel_windows_match() {
    let stack = stack_from_i8_intervals(&[(2, 5), (4, 8)]);

    assert_eq!(
        serial_windows(&stack, 0, 10, 3),
        parallel_windows(&stack, 0, 10, 3),
    );
}

#[test]
fn windows_are_unit_stepped_and_fully_contained() {
    let stack = stack_from_i8_intervals(&[]);

    assert_eq!(
        serial_windows(&stack, 2, 7, 3),
        vec![(2, 5), (3, 6), (4, 7)],
    );

    assert_eq!(
        parallel_windows(&stack, 2, 7, 3),
        vec![(2, 5), (3, 6), (4, 7)],
    );
}

#[test]
fn invalid_window_geometries_are_empty_in_both_modes() {
    let stack = stack_from_i8_intervals(&[]);

    for (from, to, len) in [(5, 5, 1), (5, 3, 1), (0, 3, 0), (0, 3, 4)] {
        assert!(serial_windows(&stack, from, to, len).is_empty());
        assert!(parallel_windows(&stack, from, to, len).is_empty());
    }
}

#[test]
fn serial_and_parallel_window_runs_match() {
    let stack = stack_from_i8_intervals(&[(1, 4), (3, 6), (8, 10)]);

    let serial: Vec<_> = stack.iter_windows(0, 11, 5).map(run_bounds).collect();

    let parallel: Vec<_> = stack.par_iter_windows(0, 11, 5).map(run_bounds).collect();

    assert_eq!(serial, parallel);
}
