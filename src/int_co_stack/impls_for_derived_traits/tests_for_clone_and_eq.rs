use std::sync::Arc;

use crate::int_co_stack::impls_for_construction::test_support::stack_from_intervals;

#[test]
fn clone_preserves_canonical_state() {
    let stack = stack_from_intervals(&[(0, 3), (1, 4), (6, 8)]);
    let cloned = stack.clone();

    assert_eq!(cloned.change_points, stack.change_points);
    assert_eq!(cloned.height_stats, stack.height_stats);
    assert_eq!(cloned, stack);

    assert!(Arc::ptr_eq(&cloned.change_points, &stack.change_points));
}

#[test]
fn clone_does_not_copy_lazy_covered_cache() {
    let stack = stack_from_intervals(&[(0, 3), (1, 4), (6, 8)]);

    assert!(stack.covered.get().is_none());

    let _ = stack.covered();

    assert!(stack.covered.get().is_some());

    let cloned = stack.clone();

    assert!(cloned.covered.get().is_none());
    assert_eq!(cloned, stack);
}

#[test]
fn equality_ignores_lazy_covered_cache_state() {
    let lhs = stack_from_intervals(&[(0, 3), (1, 4), (6, 8)]);
    let rhs = lhs.clone();

    let _ = lhs.covered();

    assert!(lhs.covered.get().is_some());
    assert!(rhs.covered.get().is_none());

    assert_eq!(lhs, rhs);
}

#[test]
fn stacks_with_different_change_points_are_not_equal() {
    let lhs = stack_from_intervals(&[(0, 3)]);
    let rhs = stack_from_intervals(&[(0, 4)]);

    assert_ne!(lhs, rhs);
}
