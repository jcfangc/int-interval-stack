use super::*;
use crate::{
    change_point::test_support::{cp, oracle_points},
    height_stats::test_support::height_stats_from_points,
    int_co_stack::{
        impls_for_construction::test_support::{endpoints_from, ep, points_from_endpoints},
        test_support::{intervals_strategy, prop_assert_canonical},
    },
};
use proptest::prelude::*;

#[inline]
fn assert_parts_eq(parts: &StackParts<i32>, expected: Vec<ChangePoint<i32>>) {
    assert_eq!(parts.points, expected);
    assert_eq!(parts.height_stats, height_stats_from_points(&expected));
}

#[test]
fn empty_endpoints_build_empty_points() {
    let parts = build_parts_from_endpoints::<i32>(Vec::new());

    assert_parts_eq(&parts, Vec::new());
}

#[test]
fn single_interval_builds_enter_and_leave_points() {
    let parts = build_parts_from_endpoints(vec![
        ep(10, EndpointKind::Leave),
        ep(3, EndpointKind::Enter),
    ]);

    assert_parts_eq(&parts, vec![cp(3, 1), cp(10, 0)]);
}

#[test]
fn adjacent_intervals_do_not_emit_redundant_boundary() {
    let parts = build_parts_from_endpoints(endpoints_from(&[(0, 5), (5, 10)]));

    assert_parts_eq(&parts, vec![cp(0, 1), cp(10, 0)]);
}

#[test]
fn nested_intervals_emit_height_changes() {
    let parts = build_parts_from_endpoints(endpoints_from(&[(1, 5), (2, 4)]));

    assert_parts_eq(&parts, vec![cp(1, 1), cp(2, 2), cp(4, 1), cp(5, 0)]);
}

#[test]
fn identical_intervals_raise_height_by_multiplicity() {
    let parts = build_parts_from_endpoints(endpoints_from(&[(1, 4), (1, 4)]));

    assert_parts_eq(&parts, vec![cp(1, 2), cp(4, 0)]);
}

#[test]
fn equal_enter_and_leave_at_same_coordinate_cancel() {
    let parts = build_parts_from_endpoints(endpoints_from(&[(0, 10), (3, 5), (5, 7)]));

    assert_parts_eq(&parts, vec![cp(0, 1), cp(3, 2), cp(7, 1), cp(10, 0)]);
}

#[test]
fn canceled_boundary_preserves_height_stats() {
    let parts = build_parts_from_endpoints(endpoints_from(&[(0, 10), (3, 5), (5, 7)]));
    let expected = vec![cp(0, 1), cp(3, 2), cp(7, 1), cp(10, 0)];

    assert_eq!(parts.points, expected);
    assert_eq!(parts.height_stats, height_stats_from_points(&expected));
}

#[test]
fn empty_input_has_default_height_stats() {
    let parts = build_parts_from_endpoints::<i32>(Vec::new());

    assert_eq!(parts.height_stats, HeightStats::default());
}

#[test]
#[should_panic(expected = "valid intervals must never produce a negative stack height")]
fn malformed_events_that_go_negative_panic() {
    let _ = build_parts_from_endpoints(vec![ep(0, EndpointKind::Leave)]);
}

proptest! {
    #[test]
    fn build_points_matches_oracle_for_valid_half_open_intervals(
        intervals in intervals_strategy(0..96)
    ) {
        let endpoints = endpoints_from(&intervals);
        let parts = build_parts_from_endpoints(endpoints);
        let expected = oracle_points(&intervals);

        prop_assert_eq!(&parts.points, &expected);
        prop_assert_eq!(parts.height_stats, height_stats_from_points(&expected));
        prop_assert_canonical(&parts.points)?;
    }

    #[test]
    fn endpoint_order_does_not_affect_result(
        intervals in intervals_strategy(0..96)
    ) {
        let endpoints = endpoints_from(&intervals);
        let mut reversed = endpoints.clone();
        reversed.reverse();

        prop_assert_eq!(
            points_from_endpoints(endpoints),
            points_from_endpoints(reversed),
        );
    }

    #[test]
    fn endpoint_order_does_not_affect_height_stats(
        intervals in intervals_strategy(0..96)
    ) {
        let endpoints = endpoints_from(&intervals);
        let mut reversed = endpoints.clone();
        reversed.reverse();

        prop_assert_eq!(
            build_parts_from_endpoints(endpoints).height_stats,
            build_parts_from_endpoints(reversed).height_stats,
        );
    }
}
