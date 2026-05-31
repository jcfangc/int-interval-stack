use crate::int_co_stack::test_support::*;
use proptest::prelude::*;

#[test]
fn intersects_can_succeed_even_if_query_start_is_uncovered() {
    let stack = stack_from_intervals(&[(5, 8)]);
    assert!(stack.intersects_interval(iv(3, 6)));
}

#[test]
fn contains_fails_if_query_crosses_an_internal_gap() {
    let stack = stack_from_intervals(&[(0, 2), (4, 6)]);
    assert!(!stack.contains_interval(iv(1, 5)));
}

proptest! {
    #[test]
    fn predicates_match_naive_oracles(
        intervals in intervals_strategy(0..96),
        query in interval_strategy(),
    ) {
        let stack = stack_from_intervals(&intervals);
        let q = iv(query.0, query.1);

        prop_assert_eq!(stack.intersects_interval(q), oracle_intersects(&intervals, query));
        prop_assert_eq!(stack.contains_interval(q), oracle_contains(&intervals, query));

        if query.0 < query.1 && stack.contains_interval(q) {
            prop_assert!(stack.intersects_interval(q));
        }
    }
}
