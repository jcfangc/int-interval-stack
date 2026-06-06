use super::*;
use crate::{change_point::test_support::oracle_points, int_co_stack::test_support::*};
use int_interval::I32CO;
use proptest::prelude::*;

#[test]
fn default_stack_is_empty_and_zero_everywhere() {
    let stack = IntCOStack::<I32CO>::default();
    assert!(stack.covered().is_empty());
    assert!(stack.change_points().is_empty());
    assert_eq!(stack.height_stats().max_height(), 0);
    assert_eq!(stack.height_at(0), 0);
    assert!(!stack.covered().contains_point(0));
}

#[test]
fn half_open_boundaries_are_observed_by_height_queries() {
    let stack = stack_from_intervals(&[(0, 5), (5, 10), (2, 7)]);

    assert_eq!(stack.height_at(-1), 0);
    assert_eq!(stack.height_at(0), 1);
    assert_eq!(stack.height_at(2), 2);
    assert_eq!(stack.height_at(5), 2);
    assert_eq!(stack.height_at(7), 1);
    assert_eq!(stack.height_at(10), 0);
    assert_eq!(stack.height_stats().max_height(), 2);
}

#[test]
fn covered_returns_union_of_positive_height_regions() {
    let stack = stack_from_intervals(&[(1, 5), (3, 8), (10, 12)]);

    assert_eq!(
        stack.covered().iter_intervals().collect::<Vec<_>>(),
        vec![iv_i32(1, 8), iv_i32(10, 12)]
    );
}

proptest! {
    #[test]
    fn accessors_match_oracles(
        intervals in intervals_strategy(0..96),
        x in -32i32..=32,
    ) {
        let stack = stack_from_intervals(&intervals);
        let expected_points = oracle_points(&intervals);
        let expected_max = expected_points.iter().map(|p| p.height_after).max().unwrap_or(0);

        prop_assert_eq!(stack.change_points(), expected_points.as_slice());
        prop_assert_eq!(stack.height_at(x), naive_height_at(&intervals, x));
        prop_assert_eq!(stack.covered().contains_point(x), naive_height_at(&intervals, x) != 0);
        prop_assert_eq!(stack.covered().is_empty(), expected_points.is_empty());
        prop_assert_eq!(stack.height_stats().max_height(), expected_max);
    }
}
