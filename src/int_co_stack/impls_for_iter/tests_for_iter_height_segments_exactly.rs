use crate::{
    change_point::test_support::cp,
    int_co_stack::{
        impls_for_construction::test_support::stack_from_points, test_support::collect_segments,
    },
};

#[test]
fn exactly_zero_uses_empty_fast_path_without_covered_cache() {
    let stack = stack_from_points(vec![cp(0, 1), cp(3, 2), cp(6, 0)]);

    assert!(stack.covered.get().is_none());

    assert_eq!(
        collect_segments(stack.iter_height_segments_exactly(0)),
        vec![],
    );

    assert!(stack.covered.get().is_none());
}

#[test]
fn exactly_below_min_positive_uses_empty_fast_path_without_covered_cache() {
    let stack = stack_from_points(vec![cp(0, 2), cp(3, 0), cp(5, 2), cp(8, 0)]);

    assert!(stack.covered.get().is_none());

    assert_eq!(
        collect_segments(stack.iter_height_segments_exactly(1)),
        vec![],
    );

    assert!(stack.covered.get().is_none());
}

#[test]
fn exactly_above_max_uses_empty_fast_path_without_covered_cache() {
    let stack = stack_from_points(vec![cp(0, 1), cp(3, 2), cp(6, 0)]);

    assert!(stack.covered.get().is_none());

    assert_eq!(
        collect_segments(stack.iter_height_segments_exactly(3)),
        vec![],
    );

    assert!(stack.covered.get().is_none());
}

#[test]
fn exactly_uniform_positive_height_uses_covered_fast_path() {
    let stack = stack_from_points(vec![cp(0, 2), cp(3, 0), cp(5, 2), cp(8, 0)]);

    assert!(stack.covered.get().is_none());

    assert_eq!(
        collect_segments(stack.iter_height_segments_exactly(2)),
        vec![((0, 3), 2), ((5, 8), 2)],
    );

    assert!(stack.covered.get().is_some());
}

#[test]
fn exactly_non_uniform_min_height_scans_change_points_without_covered_cache() {
    let stack = stack_from_points(vec![cp(0, 1), cp(2, 3), cp(5, 1), cp(7, 0)]);

    assert!(stack.covered.get().is_none());

    assert_eq!(
        collect_segments(stack.iter_height_segments_exactly(1)),
        vec![((0, 2), 1), ((5, 7), 1)],
    );

    assert!(stack.covered.get().is_none());
}

#[test]
fn exactly_non_uniform_middle_height_scans_change_points_without_covered_cache() {
    let stack = stack_from_points(vec![cp(0, 1), cp(2, 3), cp(5, 2), cp(7, 0)]);

    assert!(stack.covered.get().is_none());

    assert_eq!(
        collect_segments(stack.iter_height_segments_exactly(2)),
        vec![((5, 7), 2)],
    );

    assert!(stack.covered.get().is_none());
}

#[test]
fn exactly_non_uniform_max_height_scans_change_points_without_covered_cache() {
    let stack = stack_from_points(vec![cp(0, 1), cp(2, 3), cp(5, 2), cp(7, 0)]);

    assert!(stack.covered.get().is_none());

    assert_eq!(
        collect_segments(stack.iter_height_segments_exactly(3)),
        vec![((2, 5), 3)],
    );

    assert!(stack.covered.get().is_none());
}

#[test]
fn exactly_positive_height_on_empty_stack_yields_empty_without_covered_cache() {
    let stack = stack_from_points(vec![]);

    assert!(stack.covered.get().is_none());

    assert_eq!(
        collect_segments(stack.iter_height_segments_exactly(1)),
        vec![],
    );

    assert!(stack.covered.get().is_none());
}
