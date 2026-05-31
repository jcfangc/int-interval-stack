use super::*;
use crate::int_co_stack::test_support::*;
use proptest::prelude::*;

#[test]
fn flush_on_empty_is_noop() {
    let mut acc = StackBuildAcc::<i32>::new();
    acc.flush();
    assert!(acc.endpoints.is_empty());
    assert!(acc.levels.is_empty());
}

#[test]
fn push_interval_flushes_at_batch_boundary() {
    let mut acc = StackBuildAcc::<i32>::new();
    let mut intervals = Vec::new();

    for i in 0..(BATCH_SIZE - 1) as i32 {
        let iv_ = (i * 2, i * 2 + 1);
        intervals.push(iv_);
        acc.push_interval(iv(iv_.0, iv_.1));
    }

    assert_eq!(acc.endpoints.len(), (BATCH_SIZE - 1) * 2);
    assert!(acc.levels.is_empty());

    let last = (10_000, 10_001);
    intervals.push(last);
    acc.push_interval(iv(last.0, last.1));

    assert!(acc.endpoints.is_empty());
    assert_eq!(acc.levels.len(), 1);
    assert_eq!(acc.levels[0], Some(oracle_points(&intervals)));
}

#[test]
fn push_points_can_land_in_existing_none_level() {
    let mut acc = StackBuildAcc::<i32> {
        endpoints: Vec::new(),
        levels: vec![Some(vec![cp(0, 1), cp(3, 0)]), None],
    };

    acc.push_points(vec![cp(0, 1), cp(3, 0)]);

    assert_eq!(acc.levels[0], None);
    assert_eq!(acc.levels[1], Some(vec![cp(0, 2), cp(3, 0)]));
}

#[test]
fn push_points_carries_across_multiple_occupied_levels() {
    let mut acc = StackBuildAcc::<i32> {
        endpoints: Vec::new(),
        levels: vec![
            Some(vec![cp(0, 1), cp(2, 0)]),
            Some(vec![cp(1, 1), cp(3, 0)]),
        ],
    };

    acc.push_points(vec![cp(2, 1), cp(4, 0)]);

    assert_eq!(acc.levels[0], None);
    assert_eq!(acc.levels[1], None);
    assert_eq!(
        acc.levels[2],
        Some(oracle_points(&[(0, 2), (1, 3), (2, 4)]))
    );
}

#[test]
fn finish_merges_existing_levels_and_unflushed_tail() {
    let mut acc = StackBuildAcc::<i32>::new();
    acc.push_points(vec![cp(0, 1), cp(5, 0)]);
    acc.push_interval(iv(2, 4));

    assert_eq!(acc.finish(), oracle_points(&[(0, 5), (2, 4)]));
}

proptest! {
    #[test]
    fn stack_build_acc_finish_matches_oracle(
        intervals in intervals_strategy(0..(BATCH_SIZE * 3 + 17))
    ) {
        let mut acc = StackBuildAcc::<i32>::new();
        for &(s, e) in &intervals {
            acc.push_interval(iv(s, e));
        }

        let points = acc.finish();
        let expected = oracle_points(&intervals);

        prop_assert_eq!(points.clone(), expected);
        prop_assert_canonical(&points)?;
    }
}
