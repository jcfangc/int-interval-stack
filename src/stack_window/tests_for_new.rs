use crate::int_co_stack::test_support::{iv, stack_from_intervals};

use super::*;

fn assert_window_cache(
    stack: &IntCOStack<int_interval::I32CO>,
    window: (i32, i32),
    point_start: usize,
    point_end: usize,
    height_at_start: usize,
) {
    let window = StackWindow::new(stack, iv(window.0, window.1));

    assert_eq!(window.point_start, point_start);
    assert_eq!(window.point_end, point_end);
    assert_eq!(window.height_at_start, height_at_start);
}

#[test]
fn window_before_first_change_point_has_zero_start_height() {
    let stack = stack_from_intervals(&[(2, 5)]);

    // change points: (2 -> 1), (5 -> 0)
    assert_window_cache(&stack, (0, 2), 0, 0, 0);
}

#[test]
fn window_starting_at_change_point_uses_height_after_that_point() {
    let stack = stack_from_intervals(&[(2, 5)]);

    // point_start is first point strictly after start, so start == 2 skips
    // the change point at 2 and uses its height_after.
    assert_window_cache(&stack, (2, 4), 1, 1, 1);
}

#[test]
fn window_inside_positive_region_uses_previous_height() {
    let stack = stack_from_intervals(&[(2, 5)]);

    assert_window_cache(&stack, (3, 4), 1, 1, 1);
}

#[test]
fn window_excludes_change_point_at_end() {
    let stack = stack_from_intervals(&[(2, 5)]);

    // point_end is first point at or after end, so the point at 5 is not
    // inside [3, 5).
    assert_window_cache(&stack, (3, 5), 1, 1, 1);
}

#[test]
fn window_includes_change_points_strictly_inside() {
    let stack = stack_from_intervals(&[(1, 4), (3, 6)]);

    // change points:
    // 1 -> 1
    // 3 -> 2
    // 4 -> 1
    // 6 -> 0
    //
    // window [2, 5) contains points at 3 and 4 only.
    assert_window_cache(&stack, (2, 5), 1, 3, 1);
}

#[test]
fn window_after_all_change_points_has_zero_start_height() {
    let stack = stack_from_intervals(&[(1, 4), (3, 6)]);

    // all change points are before or at 6; after that height is zero.
    assert_window_cache(&stack, (6, 8), 4, 4, 0);
}
