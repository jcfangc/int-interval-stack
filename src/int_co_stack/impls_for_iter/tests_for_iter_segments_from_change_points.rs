use proptest::prelude::*;

use crate::{
    change_point::test_support::cp,
    int_co_stack::{
        impls_for_construction::test_support::stack_from_points,
        test_support::{
            collect_segments, intervals_strategy, oracle_segments, stack_from_intervals,
        },
    },
};

#[test]
fn empty_points_yield_no_segments() {
    let stack = stack_from_points(vec![]);

    assert_eq!(
        collect_segments(stack.iter_segments_from_change_points()),
        vec![],
    );
}

#[test]
fn trailing_zero_point_does_not_create_segment() {
    let stack = stack_from_points(vec![cp(0, 0)]);

    assert_eq!(
        collect_segments(stack.iter_segments_from_change_points()),
        vec![],
    );
}

#[test]
fn zero_height_gaps_are_skipped() {
    let stack = stack_from_points(vec![cp(0, 1), cp(2, 0), cp(5, 2), cp(8, 0)]);

    assert_eq!(
        collect_segments(stack.iter_segments_from_change_points()),
        vec![((0, 2), 1), ((5, 8), 2)],
    );
}

#[test]
fn positive_height_boundaries_are_preserved() {
    let stack = stack_from_points(vec![cp(0, 1), cp(2, 3), cp(5, 1), cp(7, 0)]);

    assert_eq!(
        collect_segments(stack.iter_segments_from_change_points()),
        vec![((0, 2), 1), ((2, 5), 3), ((5, 7), 1)],
    );
}

#[test]
fn adjacent_positive_regions_with_different_heights_are_not_merged() {
    let stack = stack_from_points(vec![cp(-3, 2), cp(0, 1), cp(4, 3), cp(6, 0)]);

    assert_eq!(
        collect_segments(stack.iter_segments_from_change_points()),
        vec![((-3, 0), 2), ((0, 4), 1), ((4, 6), 3)],
    );
}

proptest! {
    #[test]
    fn segments_match_oracle_for_constructed_stacks(
        intervals in intervals_strategy(0..96),
    ) {
        let stack = stack_from_intervals(&intervals);

        prop_assert_eq!(
            collect_segments(stack.iter_segments_from_change_points()),
            oracle_segments(&intervals),
        );
    }
}
