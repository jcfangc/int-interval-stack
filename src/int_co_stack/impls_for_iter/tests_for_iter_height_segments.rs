use proptest::prelude::*;

use crate::{
    change_point::test_support::cp,
    int_co_stack::{
        impls_for_construction::test_support::stack_from_points,
        test_support::{
            collect_segments, intervals_strategy, iv_i32, oracle_segments, stack_from_intervals,
        },
    },
};

#[test]
fn empty_stack_yields_no_segments() {
    let stack = stack_from_points(vec![]);

    assert_eq!(collect_segments(stack.iter_height_segments()), vec![]);
}

#[test]
fn single_positive_run_yields_one_segment() {
    let stack = stack_from_points(vec![cp(0, 2), cp(5, 0)]);

    assert_eq!(
        collect_segments(stack.iter_height_segments()),
        vec![((0, 5), 2)],
    );
}

#[test]
fn uniform_positive_height_uses_covered_shape() {
    let stack = stack_from_points(vec![cp(0, 2), cp(3, 0), cp(5, 2), cp(8, 0)]);

    assert_eq!(
        collect_segments(stack.iter_height_segments()),
        vec![((0, 3), 2), ((5, 8), 2)],
    );
}

#[test]
fn non_uniform_positive_heights_preserve_height_boundaries() {
    let stack = stack_from_points(vec![cp(0, 1), cp(2, 3), cp(5, 1), cp(7, 0)]);

    assert_eq!(
        collect_segments(stack.iter_height_segments()),
        vec![((0, 2), 1), ((2, 5), 3), ((5, 7), 1)],
    );
}

#[test]
fn zero_height_gaps_are_not_yielded() {
    let stack = stack_from_points(vec![cp(-2, 1), cp(0, 0), cp(4, 3), cp(6, 0)]);

    assert_eq!(
        collect_segments(stack.iter_height_segments()),
        vec![((-2, 0), 1), ((4, 6), 3)],
    );
}

proptest! {
    #[test]
    fn height_segments_match_oracle(
        intervals in intervals_strategy(0..96),
    ) {
        let stack = stack_from_intervals(&intervals);

        prop_assert_eq!(
            collect_segments(stack.iter_height_segments()),
            oracle_segments(&intervals),
        );
    }
}

#[test]
fn uniform_positive_height_path_initializes_covered_cache() {
    let stack = stack_from_points(vec![cp(0, 2), cp(3, 0), cp(5, 2), cp(8, 0)]);

    assert!(stack.covered.get().is_none());

    let segments = collect_segments(stack.iter_height_segments());

    assert!(stack.covered.get().is_some());
    assert_eq!(segments, vec![((0, 3), 2), ((5, 8), 2)]);
}

#[test]
fn non_uniform_positive_height_path_does_not_initialize_covered_cache() {
    let stack = stack_from_points(vec![cp(0, 1), cp(2, 3), cp(5, 1), cp(7, 0)]);

    assert!(stack.covered.get().is_none());

    let segments = collect_segments(stack.iter_height_segments());

    assert!(stack.covered.get().is_none());
    assert_eq!(segments, vec![((0, 2), 1), ((2, 5), 3), ((5, 7), 1)]);
}

#[test]
fn empty_stack_does_not_initialize_covered_cache() {
    let stack = stack_from_points(vec![]);

    assert!(stack.covered.get().is_none());

    let segments = collect_segments(stack.iter_height_segments());

    assert!(stack.covered.get().is_none());
    assert_eq!(segments, vec![]);
}

#[test]
fn non_uniform_path_preserves_boundaries_that_covered_shape_would_merge() {
    let stack = stack_from_points(vec![cp(0, 1), cp(2, 2), cp(5, 1), cp(8, 0)]);

    assert_eq!(
        stack.covered().iter_intervals().collect::<Vec<_>>(),
        vec![iv_i32(0, 8)],
    );

    assert_eq!(
        collect_segments(stack.iter_height_segments()),
        vec![((0, 2), 1), ((2, 5), 2), ((5, 8), 1)],
    );
}
