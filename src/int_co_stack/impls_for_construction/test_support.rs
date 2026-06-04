use int_interval::I32CO;

use crate::{height_stats::test_support::height_stats_from_points, int_co_stack::test_support::iv};

use super::*;

#[inline]
pub(super) fn ep<C>(at: C, kind: EndpointKind) -> Endpoint<C> {
    Endpoint { at, kind }
}

#[inline]
pub(super) fn points_from_endpoints<C>(endpoints: Vec<Endpoint<C>>) -> Vec<ChangePoint<C>>
where
    C: Default + Copy + Ord,
{
    build_parts_from_endpoints(endpoints).points
}

pub(super) fn endpoints_from(intervals: &[(i32, i32)]) -> Vec<Endpoint<i32>> {
    intervals
        .iter()
        .flat_map(|&(start, end)| [ep(start, EndpointKind::Enter), ep(end, EndpointKind::Leave)])
        .collect()
}

#[inline]
pub(crate) fn merge_points<C>(
    lhs: Vec<ChangePoint<C>>,
    rhs: Vec<ChangePoint<C>>,
) -> Vec<ChangePoint<C>>
where
    C: Default + Copy + Ord,
{
    merge_parts(&parts(lhs), &parts(rhs)).points
}

#[inline]
pub(super) fn parts<C: std::default::Default>(points: Vec<ChangePoint<C>>) -> StackParts<C> {
    let height_stats = height_stats_from_points(&points);

    StackParts {
        points,
        height_stats,
    }
}

#[inline]
pub(super) fn level_points<C>(acc: &StackBuildAcc<C>, level: usize) -> Option<&Vec<ChangePoint<C>>>
where
    C: Default + Copy + Ord,
{
    acc.levels
        .get(level)
        .and_then(Option::as_ref)
        .map(|parts| &parts.points)
}

#[inline]
pub(crate) fn stack_from_intervals(intervals: &[(i32, i32)]) -> IntCOStack<I32CO> {
    intervals.iter().copied().map(|(s, e)| iv(s, e)).collect()
}

#[inline]
pub(crate) fn stack_from_points(points: Vec<ChangePoint<i32>>) -> IntCOStack<I32CO> {
    let covered = covered_from_change_points::<I32CO>(&points);

    IntCOStack {
        change_points: Arc::from(points.clone().into_boxed_slice()),
        covered,
        height_stats: height_stats_from_points(&points),
    }
}
