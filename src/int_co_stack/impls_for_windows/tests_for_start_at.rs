use int_interval::{I8CO, U8CO};

use super::start_at;

#[test]
fn zero_index_returns_from_directly() {
    assert_eq!(start_at::<I8CO>(-128, 0), Some(-128));
    assert_eq!(start_at::<U8CO>(0, 0), Some(0));
}

#[test]
fn positive_index_advances_start_by_index() {
    assert_eq!(start_at::<I8CO>(-10, 7), Some(-3));
    assert_eq!(start_at::<U8CO>(10, 7), Some(17));
}

#[test]
fn signed_coord_allows_large_offset_when_result_is_representable() {
    // 200 cannot be represented as i8, but [-128, 72) is a valid I8CO.
    // This protects the implementation from being simplified to:
    //
    // from.checked_add_from(index)
    assert_eq!(start_at::<I8CO>(-128, 200), Some(72));
}

#[test]
fn returns_none_when_offset_exceeds_measure_type() {
    assert_eq!(start_at::<I8CO>(-128, 256), None);
    assert_eq!(start_at::<U8CO>(0, 256), None);
}

#[test]
fn returns_none_when_target_coordinate_is_not_representable() {
    assert_eq!(start_at::<I8CO>(100, 40), None);
    assert_eq!(start_at::<U8CO>(250, 10), None);
}
