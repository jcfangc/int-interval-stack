use super::*;
use crate::{
    change_point::test_support::{cp, oracle_points},
    int_co_stack::{
        impls_for_construction::test_support::{merge_points, parts},
        test_support::*,
    },
    stack_height_stats::test_support::height_stats_from_points,
};
use proptest::prelude::*;

#[inline]
fn assert_merge_eq(
    lhs: Vec<ChangePoint<i32>>,
    rhs: Vec<ChangePoint<i32>>,
    expected: Vec<ChangePoint<i32>>,
) {
    let merged = merge_parts(&parts(lhs), &parts(rhs));

    assert_eq!(merged.points, expected);
    assert_eq!(merged.height_stats, height_stats_from_points(&expected));
}

#[test]
fn empty_inputs_merge_to_empty() {
    assert_merge_eq(Vec::new(), Vec::new(), Vec::new());
}

#[test]
fn empty_and_non_empty_merge_to_other_side() {
    assert_merge_eq(
        Vec::new(),
        vec![cp(0, 1), cp(3, 0)],
        vec![cp(0, 1), cp(3, 0)],
    );
}

#[test]
fn boundaries_that_do_not_change_sum_are_omitted() {
    assert_merge_eq(
        vec![cp(0, 1), cp(5, 0)],
        vec![cp(5, 1), cp(10, 0)],
        vec![cp(0, 1), cp(10, 0)],
    );
}

#[test]
fn merge_covers_lhs_rhs_equal_and_remainder_paths() {
    assert_merge_eq(
        vec![cp(0, 1), cp(3, 2), cp(8, 0)],
        vec![cp(1, 1), cp(3, 0), cp(6, 1), cp(10, 0)],
        vec![cp(0, 1), cp(1, 2), cp(6, 3), cp(8, 1), cp(10, 0)],
    );
}

#[test]
fn merge_preserves_height_stats_when_boundaries_are_omitted() {
    let lhs = vec![cp(0, 1), cp(5, 0)];
    let rhs = vec![cp(5, 1), cp(10, 0)];
    let expected = vec![cp(0, 1), cp(10, 0)];

    let merged = merge_parts(&parts(lhs), &parts(rhs));

    assert_eq!(merged.points, expected);
    assert_eq!(merged.height_stats, height_stats_from_points(&expected));
}

#[test]
fn empty_inputs_have_default_height_stats() {
    let merged = merge_parts(&parts::<i32>(Vec::new()), &parts::<i32>(Vec::new()));

    assert!(merged.points.is_empty());
    assert_eq!(merged.height_stats, StackHeightStats::default());
}

#[test]
#[should_panic(expected = "stack height overflow")]
fn overflow_panics() {
    let lhs = vec![cp(0, usize::MAX), cp(1, 0)];
    let rhs = vec![cp(0, 1), cp(1, 0)];

    let _ = merge_points(lhs, rhs);
}

proptest! {
    #[test]
    fn merge_points_matches_oracle(
        lhs in intervals_strategy(0..64),
        rhs in intervals_strategy(0..64),
    ) {
        let lhs_points = oracle_points(&lhs);
        let rhs_points = oracle_points(&rhs);

        let merged = merge_parts(
            &parts(lhs_points),
            &parts(rhs_points),
        );

        let mut both = lhs.clone();
        both.extend_from_slice(&rhs);
        let expected = oracle_points(&both);

        prop_assert_eq!(&merged.points, &expected);
        prop_assert_eq!(merged.height_stats, height_stats_from_points(&expected));
        prop_assert_canonical(&merged.points)?;
    }

    #[test]
    fn merge_points_is_commutative(
        lhs in intervals_strategy(0..64),
        rhs in intervals_strategy(0..64),
    ) {
        let lhs_points = oracle_points(&lhs);
        let rhs_points = oracle_points(&rhs);

        let lhs_rhs = merge_parts(&parts(lhs_points.clone()), &parts(rhs_points.clone()));
        let rhs_lhs = merge_parts(&parts(rhs_points), &parts(lhs_points));

        prop_assert_eq!(&lhs_rhs.points, &rhs_lhs.points);
        prop_assert_eq!(lhs_rhs.height_stats, rhs_lhs.height_stats);
    }

    #[test]
    fn merge_points_is_associative(
        a in intervals_strategy(0..32),
        b in intervals_strategy(0..32),
        c in intervals_strategy(0..32),
    ) {
        let ap = parts(oracle_points(&a));
        let bp = parts(oracle_points(&b));
        let cp_ = parts(oracle_points(&c));

        let ab = merge_parts(&ap, &bp);
        let bc = merge_parts(&bp, &cp_);

        let ab_c = merge_parts(&ab, &cp_);
        let a_bc = merge_parts(&ap, &bc);

        prop_assert_eq!(&ab_c.points, &a_bc.points);
        prop_assert_eq!(ab_c.height_stats, a_bc.height_stats);
    }
}
