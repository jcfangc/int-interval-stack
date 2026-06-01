use crate::int_co_stack::test_support::height_stats_from_points;

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
pub(super) fn merge_points<C>(
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
