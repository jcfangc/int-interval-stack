use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeightSegment<I>
where
    I: IntCO,
{
    /// Closed-open interval on which the stack height is constant.
    pub interval: I,

    /// Stack height throughout `interval`.
    ///
    /// Values yielded by `IntCOStack` height-segment iterators are always
    /// positive. The type itself does not enforce that invariant.
    pub height: usize,
}
