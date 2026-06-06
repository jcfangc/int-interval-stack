use crate::{
    change_point::test_support::cp,
    int_co_stack::{
        impls_for_construction::test_support::stack_from_points, test_support::collect_segments,
    },
};

#[test]
fn at_least_above_max_uses_empty_fast_path_without_covered_cache() {
    let stack = stack_from_points(vec![cp(0, 1), cp(3, 2), cp(6, 0)]);

    assert!(stack.covered.get().is_none());

    assert_eq!(
        collect_segments(stack.iter_height_segments_at_least(3)),
        vec![],
    );

    assert!(stack.covered.get().is_none());
}

#[test]
fn at_least_zero_reuses_all_segments_path_for_uniform_stack() {
    let stack = stack_from_points(vec![cp(0, 2), cp(3, 0), cp(5, 2), cp(8, 0)]);

    assert!(stack.covered.get().is_none());

    assert_eq!(
        collect_segments(stack.iter_height_segments_at_least(0)),
        vec![((0, 3), 2), ((5, 8), 2)],
    );

    assert!(stack.covered.get().is_some());
}

#[test]
fn at_least_min_positive_reuses_all_segments_path_for_non_uniform_stack() {
    let stack = stack_from_points(vec![cp(0, 1), cp(2, 3), cp(5, 1), cp(7, 0)]);

    assert!(stack.covered.get().is_none());

    assert_eq!(
        collect_segments(stack.iter_height_segments_at_least(1)),
        vec![((0, 2), 1), ((2, 5), 3), ((5, 7), 1)],
    );

    assert!(stack.covered.get().is_none());
}

#[test]
fn at_least_between_min_and_max_scans_change_points_without_covered_cache() {
    let stack = stack_from_points(vec![cp(0, 1), cp(2, 3), cp(5, 1), cp(7, 0)]);

    assert!(stack.covered.get().is_none());

    assert_eq!(
        collect_segments(stack.iter_height_segments_at_least(2)),
        vec![((2, 5), 3)],
    );

    assert!(stack.covered.get().is_none());
}

#[test]
fn at_least_max_keeps_only_peak_segments_without_covered_cache() {
    let stack = stack_from_points(vec![cp(0, 1), cp(2, 3), cp(5, 2), cp(7, 0)]);

    assert!(stack.covered.get().is_none());

    assert_eq!(
        collect_segments(stack.iter_height_segments_at_least(3)),
        vec![((2, 5), 3)],
    );

    assert!(stack.covered.get().is_none());
}
