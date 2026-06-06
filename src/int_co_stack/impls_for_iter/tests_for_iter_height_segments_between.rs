use crate::{
    change_point::test_support::cp,
    int_co_stack::{
        impls_for_construction::test_support::stack_from_points, test_support::collect_segments,
    },
};

#[test]
fn between_min_greater_than_max_uses_empty_fast_path_without_covered_cache() {
    let stack = stack_from_points(vec![cp(0, 1), cp(3, 2), cp(6, 0)]);

    assert!(stack.covered.get().is_none());

    assert_eq!(
        collect_segments(stack.iter_height_segments_between(3, 2)),
        vec![],
    );

    assert!(stack.covered.get().is_none());
}

#[test]
fn between_empty_stack_uses_empty_fast_path_without_covered_cache() {
    let stack = stack_from_points(vec![]);

    assert!(stack.covered.get().is_none());

    assert_eq!(
        collect_segments(stack.iter_height_segments_between(1, 3)),
        vec![],
    );

    assert!(stack.covered.get().is_none());
}

#[test]
fn between_below_positive_range_uses_empty_fast_path_without_covered_cache() {
    let stack = stack_from_points(vec![cp(0, 2), cp(3, 4), cp(6, 0)]);

    assert!(stack.covered.get().is_none());

    assert_eq!(
        collect_segments(stack.iter_height_segments_between(0, 1)),
        vec![],
    );

    assert!(stack.covered.get().is_none());
}

#[test]
fn between_above_positive_range_uses_empty_fast_path_without_covered_cache() {
    let stack = stack_from_points(vec![cp(0, 1), cp(3, 2), cp(6, 0)]);

    assert!(stack.covered.get().is_none());

    assert_eq!(
        collect_segments(stack.iter_height_segments_between(3, 5)),
        vec![],
    );

    assert!(stack.covered.get().is_none());
}

#[test]
fn between_full_range_reuses_all_segments_path_for_uniform_stack() {
    let stack = stack_from_points(vec![cp(0, 2), cp(3, 0), cp(5, 2), cp(8, 0)]);

    assert!(stack.covered.get().is_none());

    assert_eq!(
        collect_segments(stack.iter_height_segments_between(1, 2)),
        vec![((0, 3), 2), ((5, 8), 2)],
    );

    assert!(stack.covered.get().is_some());
}

#[test]
fn between_full_range_reuses_all_segments_path_for_non_uniform_stack() {
    let stack = stack_from_points(vec![cp(0, 1), cp(2, 3), cp(5, 1), cp(7, 0)]);

    assert!(stack.covered.get().is_none());

    assert_eq!(
        collect_segments(stack.iter_height_segments_between(1, 3)),
        vec![((0, 2), 1), ((2, 5), 3), ((5, 7), 1)],
    );

    assert!(stack.covered.get().is_none());
}

#[test]
fn between_zero_min_is_treated_as_positive_min_when_filtering() {
    let stack = stack_from_points(vec![cp(0, 1), cp(2, 3), cp(5, 2), cp(7, 0)]);

    assert!(stack.covered.get().is_none());

    assert_eq!(
        collect_segments(stack.iter_height_segments_between(0, 2)),
        vec![((0, 2), 1), ((5, 7), 2)],
    );

    assert!(stack.covered.get().is_none());
}

#[test]
fn between_strict_inner_range_scans_change_points_without_covered_cache() {
    let stack = stack_from_points(vec![cp(0, 1), cp(2, 2), cp(5, 4), cp(8, 3), cp(10, 0)]);

    assert!(stack.covered.get().is_none());

    assert_eq!(
        collect_segments(stack.iter_height_segments_between(2, 3)),
        vec![((2, 5), 2), ((8, 10), 3)],
    );

    assert!(stack.covered.get().is_none());
}

#[test]
fn between_single_height_range_scans_change_points_without_covered_cache() {
    let stack = stack_from_points(vec![cp(0, 1), cp(2, 3), cp(5, 2), cp(7, 0)]);

    assert!(stack.covered.get().is_none());

    assert_eq!(
        collect_segments(stack.iter_height_segments_between(2, 2)),
        vec![((5, 7), 2)],
    );

    assert!(stack.covered.get().is_none());
}
