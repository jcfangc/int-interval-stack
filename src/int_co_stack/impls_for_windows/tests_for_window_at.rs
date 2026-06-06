use int_interval::I8CO;

use crate::int_co_stack::impls_for_windows::test_support::window_bounds;

use super::*;

fn empty_stack() -> IntCOStack<I8CO> {
    Vec::<I8CO>::new().into_iter().collect()
}

#[test]
fn zero_index_starts_at_from() {
    let stack = empty_stack();

    let window = window_at(&stack, 2, 3, 0).unwrap();

    assert_eq!(window_bounds(window), (2, 5));
}

#[test]
fn positive_index_advances_window_start() {
    let stack = empty_stack();

    let window = window_at(&stack, 2, 3, 4).unwrap();

    assert_eq!(window_bounds(window), (6, 9));
}

#[test]
fn supports_large_unsigned_offset_for_signed_coordinate_when_result_fits() {
    let stack = empty_stack();

    let window = window_at(&stack, i8::MIN, 1, 200).unwrap();

    assert_eq!(window_bounds(window), (72, 73));
}

#[test]
fn returns_none_when_index_does_not_fit_measure_type() {
    let stack = empty_stack();

    assert!(window_at(&stack, 0, 1, 256).is_none());
}

#[test]
fn returns_none_when_shifted_start_cannot_be_represented() {
    let stack = empty_stack();

    assert!(window_at(&stack, 100, 1, 100).is_none());
}

#[test]
fn returns_none_when_window_end_cannot_be_represented() {
    let stack = empty_stack();

    assert!(window_at(&stack, 126, 3, 0).is_none());
}

#[test]
fn returns_none_when_len_is_zero() {
    let stack = empty_stack();

    assert!(window_at(&stack, 0, 0, 0).is_none());
}
