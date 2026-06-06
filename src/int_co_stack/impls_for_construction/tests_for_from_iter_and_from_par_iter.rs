// int_co_stack/impls_for_construction/tests_for_from_iter_and_from_par_iter.rs
use super::*;
use crate::{
    change_point::test_support::{cp, oracle_points},
    int_co_stack::test_support::*,
};
use int_interval::I32CO;
use proptest::prelude::*;

#[test]
fn from_iter_handles_many_batches_of_identical_intervals() {
    let n = BATCH_SIZE * 3 + 5;
    let input = vec![iv_i32(0, 10); n];
    let stack: IntCOStack<I32CO> = input.into_iter().collect();

    assert_eq!(stack.change_points(), &[cp(0, n), cp(10, 0)]);
    assert_eq!(stack.height_stats().max_height(), n);
}

proptest! {
    #[test]
    fn par_collect_matches_seq_collect(intervals in intervals_strategy(0..256)) {
        let seq: IntCOStack<I32CO> =
            intervals.iter().copied().map(|(s, e)| iv_i32(s, e)).collect();

        let par: IntCOStack<I32CO> =
            intervals.iter().copied().map(|(s, e)| iv_i32(s, e)).collect::<Vec<_>>()
                .into_par_iter()
                .collect();

        prop_assert_eq!(seq.change_points(), par.change_points());
        prop_assert_eq!(seq.height_stats().max_height(), par.height_stats().max_height());
    }

    #[test]
    fn seq_collect_matches_public_oracle(intervals in intervals_strategy(0..256)) {
        let stack: IntCOStack<I32CO> =
            intervals.iter().copied().map(|(s, e)| iv_i32(s, e)).collect();

        let expected = oracle_points(&intervals);

        prop_assert_eq!(stack.change_points(), expected.as_slice());
    }
}
