use crate::int_co_stack::test_support::*;
use proptest::prelude::*;

#[test]
fn zero_threshold_edges_behave_as_current_implementation() {
    let stack = stack_from_intervals(&[(0, 2), (1, 3)]);

    assert_eq!(
        collect_segments(stack.iter_intervals()),
        collect_segments(stack.iter_intervals_at_least(0)),
    );
    assert!(collect_segments(stack.iter_intervals_at_most(0)).is_empty());
    assert!(collect_segments(stack.iter_intervals_exactly(0)).is_empty());
    assert!(collect_segments(stack.iter_intervals_between(3, 2)).is_empty());
}

#[test]
fn peak_intervals_return_all_max_segments() {
    let stack = stack_from_intervals(&[(0, 2), (1, 3), (5, 7), (6, 8)]);
    assert_eq!(
        collect_segments(stack.peak_intervals()),
        vec![((1, 2), 2), ((6, 7), 2)],
    );
}

proptest! {
    #[test]
    fn iterators_match_oracle_segments(
        intervals in intervals_strategy(0..96),
        min in 0usize..8,
        max in 0usize..8,
        target in 0usize..8,
    ) {
        let stack = stack_from_intervals(&intervals);
        let segs = oracle_segments(&intervals);
        let max_h = segs.iter().map(|(_, h)| *h).max().unwrap_or(0);

        prop_assert_eq!(collect_segments(stack.iter_intervals()), segs);

        prop_assert_eq!(
            collect_segments(stack.iter_intervals_at_least(min)),
            oracle_segments(&intervals)
                .into_iter()
                .filter(|(_, h)| *h >= min)
                .collect::<Vec<_>>()
        );

        prop_assert_eq!(
            collect_segments(stack.iter_intervals_at_most(max)),
            oracle_segments(&intervals)
                .into_iter()
                .filter(|(_, h)| *h <= max)
                .collect::<Vec<_>>()
        );

        prop_assert_eq!(
            collect_segments(stack.iter_intervals_exactly(target)),
            oracle_segments(&intervals)
                .into_iter()
                .filter(|(_, h)| *h == target)
                .collect::<Vec<_>>()
        );

        prop_assert_eq!(
            collect_segments(stack.iter_intervals_between(min, max)),
            oracle_segments(&intervals)
                .into_iter()
                .filter(|(_, h)| *h >= min && *h <= max)
                .collect::<Vec<_>>()
        );

        prop_assert_eq!(
            collect_segments(stack.peak_intervals()),
            oracle_segments(&intervals)
                .into_iter()
                .filter(|(_, h)| *h == max_h)
                .collect::<Vec<_>>()
        );
    }
}
