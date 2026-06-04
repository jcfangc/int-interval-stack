use int_interval::I32CO;
use int_interval_set::IntCOSet;
use proptest::prelude::*;

use super::*;
use crate::{
    change_point::test_support::oracle_points,
    int_co_stack::test_support::{intervals_strategy, iv},
};

fn covered_vec(points: &[ChangePoint<i32>]) -> Vec<I32CO> {
    covered_from_change_points::<I32CO>(points)
        .iter_intervals()
        .collect()
}

#[test]
fn empty_points_returns_empty_set() {
    assert_eq!(covered_vec(&[]), vec![]);
}

#[test]
fn single_positive_run_returns_one_interval() {
    let points = [
        ChangePoint {
            at: 1,
            height_after: 1,
        },
        ChangePoint {
            at: 5,
            height_after: 0,
        },
    ];

    assert_eq!(covered_vec(&points), vec![iv(1, 5)]);
}

#[test]
fn positive_height_changes_are_merged_into_one_covered_run() {
    let points = [
        ChangePoint {
            at: 1,
            height_after: 1,
        },
        ChangePoint {
            at: 3,
            height_after: 2,
        },
        ChangePoint {
            at: 6,
            height_after: 1,
        },
        ChangePoint {
            at: 9,
            height_after: 0,
        },
    ];

    assert_eq!(covered_vec(&points), vec![iv(1, 9)]);
}

#[test]
fn zero_height_gaps_split_covered_runs() {
    let points = [
        ChangePoint {
            at: 1,
            height_after: 1,
        },
        ChangePoint {
            at: 3,
            height_after: 0,
        },
        ChangePoint {
            at: 5,
            height_after: 2,
        },
        ChangePoint {
            at: 8,
            height_after: 0,
        },
    ];

    assert_eq!(covered_vec(&points), vec![iv(1, 3), iv(5, 8)]);
}

proptest! {
    #[test]
    fn covered_projection_matches_interval_set(xs in intervals_strategy(0..64)) {
        let points = oracle_points(&xs);

        let covered = covered_from_change_points::<I32CO>(&points);
        let expected: IntCOSet<I32CO> = xs
            .iter()
            .copied()
            .map(|(start, end_excl)| iv(start, end_excl))
            .collect();

        prop_assert_eq!(covered, expected);
    }
}
