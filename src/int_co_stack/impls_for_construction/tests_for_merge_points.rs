use super::*;
use crate::int_co_stack::test_support::*;
use proptest::prelude::*;

#[test]
fn empty_inputs_merge_to_empty() {
    assert_eq!(
        merge_points::<i32>(&[], &[]),
        Vec::<ChangePoint<i32>>::new()
    );
}

#[test]
fn empty_and_non_empty_merge_to_other_side() {
    let rhs = vec![cp(0, 1), cp(3, 0)];
    assert_eq!(merge_points::<i32>(&[], &rhs), rhs);
}

#[test]
fn boundaries_that_do_not_change_sum_are_omitted() {
    let lhs = vec![cp(0, 1), cp(5, 0)];
    let rhs = vec![cp(5, 1), cp(10, 0)];

    assert_eq!(merge_points(&lhs, &rhs), vec![cp(0, 1), cp(10, 0)]);
}

#[test]
fn merge_covers_lhs_rhs_equal_and_remainder_paths() {
    let lhs = vec![cp(0, 1), cp(3, 2), cp(8, 0)];
    let rhs = vec![cp(1, 1), cp(3, 0), cp(6, 1), cp(10, 0)];

    assert_eq!(
        merge_points(&lhs, &rhs),
        vec![cp(0, 1), cp(1, 2), cp(6, 3), cp(8, 1), cp(10, 0)]
    );
}

#[test]
#[should_panic(expected = "stack height overflow")]
fn overflow_panics() {
    let lhs = vec![cp(0, usize::MAX), cp(1, 0)];
    let rhs = vec![cp(0, 1), cp(1, 0)];
    let _ = merge_points(&lhs, &rhs);
}

proptest! {
    #[test]
    fn merge_points_matches_oracle(
        lhs in intervals_strategy(0..64),
        rhs in intervals_strategy(0..64),
    ) {
        let lhs_points = oracle_points(&lhs);
        let rhs_points = oracle_points(&rhs);
        let merged = merge_points(&lhs_points, &rhs_points);

        let mut both = lhs.clone();
        both.extend_from_slice(&rhs);
        let expected = oracle_points(&both);

        prop_assert_eq!(merged.clone(), expected);
        prop_assert_canonical(&merged)?;
    }

    #[test]
    fn merge_points_is_commutative(
        lhs in intervals_strategy(0..64),
        rhs in intervals_strategy(0..64),
    ) {
        let lhs_points = oracle_points(&lhs);
        let rhs_points = oracle_points(&rhs);

        prop_assert_eq!(
            merge_points(&lhs_points, &rhs_points),
            merge_points(&rhs_points, &lhs_points),
        );
    }

    #[test]
    fn merge_points_is_associative(
        a in intervals_strategy(0..32),
        b in intervals_strategy(0..32),
        c in intervals_strategy(0..32),
    ) {
        let ap = oracle_points(&a);
        let bp = oracle_points(&b);
        let cp_ = oracle_points(&c);

        let ab_c = merge_points(&merge_points(&ap, &bp), &cp_);
        let a_bc = merge_points(&ap, &merge_points(&bp, &cp_));

        prop_assert_eq!(ab_c, a_bc);
    }
}
