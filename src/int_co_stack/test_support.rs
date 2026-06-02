use super::*;
use int_interval::I32CO;
use proptest::{prelude::*, test_runner::TestCaseResult};
use std::collections::BTreeSet;

pub(crate) fn iv(start: i32, end_excl: i32) -> I32CO {
    I32CO::try_new(start, end_excl).unwrap()
}

pub(crate) fn cp(at: i32, height_after: usize) -> ChangePoint<i32> {
    ChangePoint { at, height_after }
}

pub(crate) fn naive_height_at(intervals: &[(i32, i32)], x: i32) -> usize {
    intervals.iter().filter(|&&(s, e)| s <= x && x < e).count()
}

pub(crate) fn oracle_segments(intervals: &[(i32, i32)]) -> Vec<((i32, i32), usize)> {
    oracle_points(intervals)
        .windows(2)
        .filter_map(|w| {
            let start = w[0].at;
            let end_excl = w[1].at;
            let height = w[0].height_after;

            (height != 0).then_some(((start, end_excl), height))
        })
        .collect()
}

pub(crate) fn oracle_intersects(intervals: &[(i32, i32)], query: (i32, i32)) -> bool {
    let (start, end_excl) = query;
    start < end_excl && (start..end_excl).any(|x| naive_height_at(intervals, x) != 0)
}

pub(crate) fn oracle_contains(intervals: &[(i32, i32)], query: (i32, i32)) -> bool {
    let (start, end_excl) = query;
    start >= end_excl || (start..end_excl).all(|x| naive_height_at(intervals, x) != 0)
}

pub(crate) fn collect_segments(
    iter: impl Iterator<Item = (I32CO, usize)>,
) -> Vec<((i32, i32), usize)> {
    iter.map(|(iv, h)| ((iv.start(), iv.end_excl()), h))
        .collect()
}

pub(crate) fn prop_assert_canonical(points: &[ChangePoint<i32>]) -> TestCaseResult {
    for w in points.windows(2) {
        prop_assert!(w[0].at < w[1].at);
        prop_assert_ne!(w[0].height_after, w[1].height_after);
    }
    if let Some(last) = points.last() {
        prop_assert_eq!(last.height_after, 0);
    }
    Ok(())
}

prop_compose! {
    pub(crate) fn interval_strategy()(
        start in -24i32..=24,
        len in 1i32..=24,
    ) -> (i32, i32) {
        (start, start + len)
    }
}

pub(crate) fn intervals_strategy(
    range: std::ops::Range<usize>,
) -> impl Strategy<Value = Vec<(i32, i32)>> {
    prop::collection::vec(interval_strategy(), range)
}
