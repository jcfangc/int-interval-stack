use crate::int_co_stack::test_support::{iv_i32, stack_from_intervals};

use super::*;

fn assert_run(
    window: &StackWindow<'_, int_interval::I32CO>,
    run_index: usize,
    interval: (i32, i32),
    height: usize,
) {
    let run = window.height_run_at(run_index);

    assert_eq!(
        (run.interval.start(), run.interval.end_excl(), run.height),
        (interval.0, interval.1, height),
    );
}

#[test]
fn single_run_window_without_interior_change_points() {
    let stack = stack_from_intervals(&[(2, 8)]);
    let window = StackWindow::new(&stack, iv_i32(3, 6));

    assert_eq!(window.height_run_count(), 1);
    assert_run(&window, 0, (3, 6), 1);
}

#[test]
fn window_starting_at_change_point_uses_height_after_start() {
    let stack = stack_from_intervals(&[(2, 8)]);
    let window = StackWindow::new(&stack, iv_i32(2, 5));

    assert_eq!(window.height_run_count(), 1);
    assert_run(&window, 0, (2, 5), 1);
}

#[test]
fn change_point_at_window_end_is_excluded() {
    let stack = stack_from_intervals(&[(2, 8)]);
    let window = StackWindow::new(&stack, iv_i32(3, 8));

    assert_eq!(window.height_run_count(), 1);
    assert_run(&window, 0, (3, 8), 1);
}

#[test]
fn multiple_interior_change_points_split_runs() {
    let stack = stack_from_intervals(&[(1, 4), (3, 6)]);
    let window = StackWindow::new(&stack, iv_i32(2, 5));

    assert_eq!(window.height_run_count(), 3);

    assert_run(&window, 0, (2, 3), 1);
    assert_run(&window, 1, (3, 4), 2);
    assert_run(&window, 2, (4, 5), 1);
}

#[test]
fn zero_height_gap_inside_window_is_returned_as_run() {
    let stack = stack_from_intervals(&[(1, 3), (5, 7)]);
    let window = StackWindow::new(&stack, iv_i32(2, 6));

    assert_eq!(window.height_run_count(), 3);

    assert_run(&window, 0, (2, 3), 1);
    assert_run(&window, 1, (3, 5), 0);
    assert_run(&window, 2, (5, 6), 1);
}

#[test]
fn window_after_all_change_points_has_single_zero_run() {
    let stack = stack_from_intervals(&[(1, 3)]);
    let window = StackWindow::new(&stack, iv_i32(3, 6));

    assert_eq!(window.height_run_count(), 1);
    assert_run(&window, 0, (3, 6), 0);
}
