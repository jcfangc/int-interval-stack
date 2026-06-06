use int_interval::{I8CO, UsizeCO};

use super::window_count;

#[test]
fn returns_none_for_zero_len() {
    assert_eq!(window_count::<UsizeCO>(0, 10, 0), None);
}

#[test]
fn returns_none_for_empty_or_reversed_domain() {
    assert_eq!(window_count::<UsizeCO>(5, 5, 1), None);
    assert_eq!(window_count::<UsizeCO>(6, 5, 1), None);
}

#[test]
fn returns_none_when_len_exceeds_domain_len() {
    assert_eq!(window_count::<UsizeCO>(0, 3, 4), None);
}

#[test]
fn returns_one_when_len_equals_domain_len() {
    assert_eq!(window_count::<UsizeCO>(0, 3, 3), Some(1));
}

#[test]
fn returns_domain_len_minus_len_plus_one() {
    assert_eq!(window_count::<UsizeCO>(0, 10, 3), Some(8));
    assert_eq!(window_count::<UsizeCO>(2, 7, 3), Some(3));
}

#[test]
fn works_for_signed_coordinate_intervals() {
    assert_eq!(window_count::<I8CO>(-5, 5, 3), Some(8));
    assert_eq!(window_count::<I8CO>(-5, 5, 10), Some(1));
    assert_eq!(window_count::<I8CO>(-5, 5, 11), None);
}

#[test]
fn works_near_signed_coordinate_bounds() {
    assert_eq!(window_count::<I8CO>(i8::MIN, i8::MIN + 3, 1), Some(3));
    assert_eq!(window_count::<I8CO>(i8::MAX - 3, i8::MAX, 2), Some(2));
}
