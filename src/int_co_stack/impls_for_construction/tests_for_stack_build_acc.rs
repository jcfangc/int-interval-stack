use super::*;
use crate::{
    change_point::test_support::{cp, oracle_points},
    height_stats::test_support::height_stats_from_points,
    int_co_stack::{
        impls_for_construction::test_support::{
            assert_level_eq, assert_parts_eq, level_points, parts,
        },
        test_support::*,
    },
};
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
        acc.push_interval(iv_i32(iv_.0, iv_.1));
    }

    assert_eq!(acc.endpoints.len(), (BATCH_SIZE - 1) * 2);
    assert!(acc.levels.is_empty());

    let last = (10_000, 10_001);
    intervals.push(last);
    acc.push_interval(iv_i32(last.0, last.1));

    let expected = oracle_points(&intervals);

    assert!(acc.endpoints.is_empty());
    assert_eq!(acc.levels.len(), 1);
    assert_eq!(level_points(&acc, 0), Some(&expected));
    assert_level_eq(&acc, 0, expected);
}

#[test]
fn push_points_can_land_in_existing_none_level() {
    let mut acc = StackBuildAcc::<i32> {
        endpoints: Vec::new(),
        levels: vec![Some(parts(vec![cp(0, 1), cp(3, 0)])), None],
    };

    acc.push_points(parts(vec![cp(0, 1), cp(3, 0)]));

    assert!(acc.levels[0].is_none());
    assert_level_eq(&acc, 1, vec![cp(0, 2), cp(3, 0)]);
}

#[test]
fn push_points_carries_across_multiple_occupied_levels() {
    let mut acc = StackBuildAcc::<i32> {
        endpoints: Vec::new(),
        levels: vec![
            Some(parts(vec![cp(0, 1), cp(2, 0)])),
            Some(parts(vec![cp(1, 1), cp(3, 0)])),
        ],
    };

    acc.push_points(parts(vec![cp(2, 1), cp(4, 0)]));

    let expected = oracle_points(&[(0, 2), (1, 3), (2, 4)]);

    assert!(acc.levels[0].is_none());
    assert!(acc.levels[1].is_none());
    assert_level_eq(&acc, 2, expected);
}

#[test]
fn finish_merges_existing_levels_and_unflushed_tail() {
    let mut acc = StackBuildAcc::<i32>::new();

    acc.push_points(parts(vec![cp(0, 1), cp(5, 0)]));
    acc.push_interval(iv_i32(2, 4));

    let parts = acc.finish();
    let expected = oracle_points(&[(0, 5), (2, 4)]);

    assert_parts_eq(&parts, expected);
}

#[test]
fn finish_single_existing_level_preserves_height_stats() {
    let mut acc = StackBuildAcc::<i32>::new();
    let expected = vec![cp(0, 1), cp(5, 0)];

    acc.push_points(parts(expected.clone()));

    let parts = acc.finish();

    assert_parts_eq(&parts, expected);
}

proptest! {
    #[test]
    fn stack_build_acc_finish_matches_oracle(
        intervals in intervals_strategy(0..(BATCH_SIZE * 3 + 17))
    ) {
        let mut acc = StackBuildAcc::<i32>::new();

        for &(s, e) in &intervals {
            acc.push_interval(iv_i32(s, e));
        }

        let parts = acc.finish();
        let expected = oracle_points(&intervals);

        prop_assert_eq!(&parts.points, &expected);
        prop_assert_eq!(parts.height_stats, height_stats_from_points(&expected));
        prop_assert_canonical(&parts.points)?;
    }
}
