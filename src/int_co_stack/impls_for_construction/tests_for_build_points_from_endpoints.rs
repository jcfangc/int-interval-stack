use super::*;
use crate::int_co_stack::test_support::{
    cp, intervals_strategy, oracle_points, prop_assert_canonical,
};
use proptest::prelude::*;

#[inline]
fn ep<C>(at: C, kind: EndpointKind) -> Endpoint<C> {
    Endpoint { at, kind }
}

fn endpoints_from(intervals: &[(i32, i32)]) -> Vec<Endpoint<i32>> {
    intervals
        .iter()
        .flat_map(|&(start, end)| [ep(start, EndpointKind::Enter), ep(end, EndpointKind::Leave)])
        .collect()
}

#[test]
fn empty_endpoints_build_empty_points() {
    assert_eq!(build_points_from_endpoints::<i32>(Vec::new()), []);
}

#[test]
fn single_interval_builds_enter_and_leave_points() {
    let points = build_points_from_endpoints(vec![
        ep(10, EndpointKind::Leave),
        ep(3, EndpointKind::Enter),
    ]);

    assert_eq!(points, vec![cp(3, 1), cp(10, 0)]);
}

#[test]
fn adjacent_intervals_do_not_emit_redundant_boundary() {
    let points = build_points_from_endpoints(endpoints_from(&[(0, 5), (5, 10)]));

    assert_eq!(points, vec![cp(0, 1), cp(10, 0)]);
}

#[test]
fn nested_intervals_emit_height_changes() {
    let points = build_points_from_endpoints(endpoints_from(&[(1, 5), (2, 4)]));

    assert_eq!(points, vec![cp(1, 1), cp(2, 2), cp(4, 1), cp(5, 0)]);
}

#[test]
fn identical_intervals_raise_height_by_multiplicity() {
    let points = build_points_from_endpoints(endpoints_from(&[(1, 4), (1, 4)]));

    assert_eq!(points, vec![cp(1, 2), cp(4, 0)]);
}

#[test]
fn equal_enter_and_leave_at_same_coordinate_cancel() {
    let points = build_points_from_endpoints(endpoints_from(&[(0, 10), (3, 5), (5, 7)]));

    assert_eq!(points, vec![cp(0, 1), cp(3, 2), cp(7, 1), cp(10, 0)]);
}

#[test]
#[should_panic(expected = "valid intervals must never produce a negative stack height")]
fn malformed_events_that_go_negative_panic() {
    let _ = build_points_from_endpoints(vec![ep(0, EndpointKind::Leave)]);
}

proptest! {
    #[test]
    fn build_points_matches_oracle_for_valid_half_open_intervals(
        intervals in intervals_strategy(0..96)
    ) {
        let endpoints = endpoints_from(&intervals);
        let points = build_points_from_endpoints(endpoints);
        let expected = oracle_points(&intervals);

        prop_assert_eq!(&points, &expected);
        prop_assert_canonical(&points)?;
    }

    #[test]
    fn endpoint_order_does_not_affect_result(
        intervals in intervals_strategy(0..96)
    ) {
        let endpoints = endpoints_from(&intervals);
        let mut reversed = endpoints.clone();
        reversed.reverse();

        prop_assert_eq!(
            build_points_from_endpoints(endpoints),
            build_points_from_endpoints(reversed),
        );
    }
}
