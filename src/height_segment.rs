use core::num::NonZeroUsize;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeightSegment<I>
where
    I: IntCO,
{
    /// Closed-open interval on which the positive stack height is constant.
    pub interval: I,

    /// Positive stack height throughout `interval`.
    pub height: NonZeroUsize,
}

impl<I> From<HeightSegment<I>> for HeightRun<I>
where
    I: IntCO,
{
    #[inline]
    fn from(segment: HeightSegment<I>) -> Self {
        Self {
            interval: segment.interval,
            height: segment.height.get(),
        }
    }
}
