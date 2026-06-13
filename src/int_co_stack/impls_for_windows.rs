use std::num::NonZeroUsize;

use either::Either;
use int_interval::traits::{COStartLenConstruct, IntPrimitive};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use super::*;
use crate::stack_window::{WindowCursor, WindowIter};

#[inline]
fn window_count<I>(from: I::CoordType, to: I::CoordType, len: I::MeasureType) -> Option<usize>
where
    I: IntCO + COStartLenConstruct,
    I::MeasureType: TryInto<usize>,
{
    if len == I::MeasureType::zero() {
        return None;
    }

    let domain = I::try_new(from, to)?;
    let remaining = domain.len().checked_sub(len)?;
    let count = remaining.checked_add(I::MeasureType::one())?;

    count.try_into().ok()
}

#[inline]
fn start_at<I>(from: I::CoordType, index: usize) -> Option<I::CoordType>
where
    I: IntCO + COStartLenConstruct,
{
    if index == 0 {
        return Some(from);
    }

    let offset = I::MeasureType::checked_from(index)?;

    I::checked_from_start_len(from, offset).map(|interval| interval.end_excl())
}

#[inline]
pub(crate) fn window_at<'a, I>(
    stack: &'a IntCOStack<I>,
    from: I::CoordType,
    len: I::MeasureType,
    index: usize,
) -> Option<StackWindow<'a, I>>
where
    I: IntCO + COStartLenConstruct + Copy,
{
    let start = start_at::<I>(from, index)?;
    let interval = I::checked_from_start_len(start, len)?;

    Some(StackWindow::new(stack, interval))
}

impl<I> IntCOStack<I>
where
    I: IntCO + COStartLenConstruct + Copy,
    I::MeasureType: TryInto<usize>,
{
    /// Iterates over all fixed-length windows fully contained in `[from, to)`.
    ///
    /// Window starts advance by one coordinate unit:
    ///
    /// ```text
    /// [from,     from + len)
    /// [from + 1, from + 1 + len)
    /// ...
    /// ```
    ///
    /// Returns an empty iterator when:
    ///
    /// - `from >= to`;
    /// - `len == 0`;
    /// - `len` is greater than the measure of `[from, to)`;
    /// - the window count cannot be represented as `usize`.
    #[inline]
    pub fn iter_windows(
        &self,
        from: I::CoordType,
        to: I::CoordType,
        len: I::MeasureType,
    ) -> impl DoubleEndedIterator<Item = StackWindow<'_, I>> + ExactSizeIterator {
        let count = window_count::<I>(from, to, len).unwrap_or(0);

        let Some(count) = NonZeroUsize::new(count) else {
            return Either::Left(std::iter::empty());
        };

        let cursor = WindowCursor::new(self, from, len, count);
        Either::Right(WindowIter {
            cursor,
            total_count: count.get(),
            consumed_back: 0,
        })
    }

    /// Iterates in parallel over all fixed-length windows fully contained in
    /// `[from, to)`.
    ///
    /// The valid window-start range is represented as an indexed integer
    /// range, allowing Rayon to split the work directly without a serial
    /// producer or `par_bridge`.
    #[inline]
    pub fn par_iter_windows(
        &self,
        from: I::CoordType,
        to: I::CoordType,
        len: I::MeasureType,
    ) -> impl IndexedParallelIterator<Item = StackWindow<'_, I>>
    where
        I: Send + Sync,
    {
        let count = window_count::<I>(from, to, len).unwrap_or(0);

        (0..count).into_par_iter().map(move |index| {
            window_at(self, from, len, index)
                .expect("validated window index must produce a representable window")
        })
    }
}

// ---------------------------------------------------------------------------
// DoubleEndedIterator – lives here because it calls the module-local
// `window_at` helper.
// ---------------------------------------------------------------------------

impl<'a, I> DoubleEndedIterator for WindowIter<'a, I>
where
    I: IntCO + COStartLenConstruct + Copy,
    I::MeasureType: TryInto<usize>,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.cursor.remaining == 0 {
            return None;
        }

        let back_index = self.total_count - self.consumed_back - 1;
        self.consumed_back += 1;
        self.cursor.remaining -= 1;

        Some(
            window_at(
                self.cursor.stack,
                self.cursor.from,
                self.cursor.interval.len(),
                back_index,
            )
            .expect("back index is always valid when remaining > 0"),
        )
    }
}

#[cfg(test)]
mod test_support;

#[cfg(test)]
mod tests_for_window_count;

#[cfg(test)]
mod tests_for_start_at;

#[cfg(test)]
mod tests_for_window_at;

#[cfg(test)]
mod tests_for_iter_windows;
