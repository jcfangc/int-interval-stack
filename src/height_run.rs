use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeightRun<I>
where
    I: IntCO,
{
    /// Closed-open interval on which the stack height is constant.
    pub interval: I,

    /// Stack height throughout `interval`.
    ///
    /// This may be zero.
    pub height: usize,
}
