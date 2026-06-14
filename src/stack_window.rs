use std::num::NonZeroUsize;

use int_interval::traits::{COStartLenConstruct, IntCO, IntPrimitive};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use crate::{HeightRun, IntCOStack};

#[derive(Debug, Clone, Copy)]
pub struct StackWindow<'a, I>
where
    I: IntCO,
{
    pub(crate) stack: &'a IntCOStack<I>,
    pub(crate) interval: I,
    /// First change point strictly inside the window after `interval.start()`.
    pub(crate) point_start: usize,
    /// First change point at or after `interval.end_excl()`.
    pub(crate) point_end: usize,
    /// Stack height at `interval.start()`.
    pub(crate) height_at_start: usize,
}

impl<'a, I> StackWindow<'a, I>
where
    I: IntCO,
{
    pub(crate) fn new(stack: &'a IntCOStack<I>, interval: I) -> Self {
        let points = stack.change_points();

        let window_start = interval.start();
        let window_end = interval.end_excl();

        let point_start = points.partition_point(|point| point.at <= window_start);
        let point_end =
            point_start + points[point_start..].partition_point(|point| point.at < window_end);

        let height_at_start = point_start
            .checked_sub(1)
            .map_or(0, |index| points[index].height_after);

        Self {
            stack,
            interval,
            point_start,
            point_end,
            height_at_start,
        }
    }

    #[inline]
    pub const fn stack(&self) -> &'a IntCOStack<I> {
        self.stack
    }

    #[inline]
    pub const fn interval(&self) -> &I {
        &self.interval
    }

    /// Returns the number of constant-height runs inside this window.
    ///
    /// A window is partitioned by every stack change point strictly inside the
    /// window:
    ///
    /// ```text
    /// [window.start, p0), [p0, p1), ..., [pn, window.end)
    /// ```
    ///
    /// Therefore the number of runs is the number of interior change points plus
    /// one. Even a window with no interior change points has one run covering the
    /// whole window.
    #[inline]
    fn height_run_count(&self) -> usize {
        self.point_end - self.point_start + 1
    }

    /// Builds the constant-height run at `run_index`.
    ///
    /// `run_index` is an index into the window-local run partition, not into the
    /// global change-point array.
    ///
    /// For a window containing interior change points
    /// `points[point_start..point_end]`, the run boundaries are:
    ///
    /// - run `0`: starts at `interval.start()`;
    /// - run `i > 0`: starts at `points[point_start + i - 1].at`;
    /// - run `i < count - 1`: ends at `points[point_start + i].at`;
    /// - final run: ends at `interval.end_excl()`.
    ///
    /// Heights follow the stack height active at each run start:
    ///
    /// - run `0` uses `height_at_start`;
    /// - run `i > 0` uses the height after the preceding interior change point.
    #[inline]
    fn height_run_at(&self, run_index: usize) -> HeightRun<I> {
        debug_assert!(run_index < self.height_run_count());

        let points = self.stack.change_points();

        let point_index = self.point_start + run_index;

        let start = if run_index == 0 {
            self.interval.start()
        } else {
            points[point_index - 1].at
        };

        let end_excl = if point_index < self.point_end {
            points[point_index].at
        } else {
            self.interval.end_excl()
        };

        let height = if run_index == 0 {
            self.height_at_start
        } else {
            points[point_index - 1].height_after
        };

        HeightRun {
            // SAFETY:
            // `run_index` ranges over the partition induced by change points
            // strictly inside this window. Therefore each produced pair is a
            // non-empty closed-open interval.
            interval: unsafe { I::new_unchecked(start, end_excl) },
            height,
        }
    }
}

impl<I> StackWindow<'_, I>
where
    I: IntCO,
{
    /// Iterates over constant-height runs inside this window.
    ///
    /// Unlike `IntCOStack::iter_height_segments`, this includes zero-height
    /// runs because window-level mappings may assign a non-zero value to
    /// height zero.
    #[inline]
    pub fn iter_height_runs(
        &self,
    ) -> impl DoubleEndedIterator<Item = HeightRun<I>> + ExactSizeIterator {
        (0..self.height_run_count()).map(move |run_index| self.height_run_at(run_index))
    }
}

impl<I> StackWindow<'_, I>
where
    I: IntCO + Send + Sync,
{
    /// Iterates in parallel over constant-height runs inside this window.
    ///
    /// The run range is represented as an indexed integer range, so Rayon can
    /// split the work directly. This is mainly useful when the per-run mapping
    /// is expensive or the window contains many height changes.
    #[inline]
    pub fn par_iter_height_runs(&self) -> impl IndexedParallelIterator<Item = HeightRun<I>> {
        (0..self.height_run_count())
            .into_par_iter()
            .map(move |run_index| self.height_run_at(run_index))
    }
}

// ---------------------------------------------------------------------------
// WindowIter – sliding-window iterator
// ---------------------------------------------------------------------------

/// Iterator over sliding windows that uses incremental index advancement for
/// O(1) amortized forward steps.
///
/// Backward iteration falls back to a binary search per step.
#[derive(Debug, Clone)]
pub(crate) struct WindowIter<'a, I>
where
    I: IntCO,
{
    pub(crate) stack: &'a IntCOStack<I>,
    /// Iteration domain start, stored for `DoubleEndedIterator::next_back`.
    pub(crate) from: I::CoordType,
    /// Current window `[start, start + len)`.
    pub(crate) interval: I,
    /// First change point strictly inside the window after `interval.start()`.
    pub(crate) point_start: usize,
    /// First change point at or after `interval.end_excl()`.
    pub(crate) point_end: usize,
    /// Stack height at `interval.start()`.
    pub(crate) height_at_start: usize,
    /// Number of windows remaining, including the current one.
    pub(crate) remaining: usize,
    /// Total window count (constant).
    pub(crate) total_count: usize,
    /// Number of windows already taken from the back via `next_back`.
    pub(crate) consumed_back: usize,
}

impl<'a, I> WindowIter<'a, I>
where
    I: IntCO + COStartLenConstruct + Copy,
{
    /// Positions the iterator at the first window `[from, from + len)`.
    ///
    /// Uses a binary search once to locate the initial change-point range.
    pub(crate) fn new(
        stack: &'a IntCOStack<I>,
        from: I::CoordType,
        len: I::MeasureType,
        count: NonZeroUsize,
    ) -> Self {
        let interval = I::checked_from_start_len(from, len)
            .expect("validated window count guarantees a representable first window");

        let sw = StackWindow::new(stack, interval);

        Self {
            stack,
            from,
            interval,
            point_start: sw.point_start,
            point_end: sw.point_end,
            height_at_start: sw.height_at_start,
            remaining: count.get(),
            total_count: count.get(),
            consumed_back: 0,
        }
    }

    /// Slides the window forward by one coordinate unit.
    ///
    /// Advances `point_start` and `point_end` past any change points that fall
    /// on or before the new window boundaries.
    ///
    /// # Panics
    ///
    /// Panics on overflow if `remaining` is zero (debug-only via `debug_assert`).
    #[inline]
    fn advance(&mut self) {
        debug_assert!(self.remaining > 0);

        let new_start = self
            .interval
            .start()
            .checked_next()
            .expect("remaining > 0 guarantees the next start coordinate fits");
        let new_end = self
            .interval
            .end_excl()
            .checked_next()
            .expect("remaining > 0 guarantees the next end coordinate fits");

        // SAFETY: `remaining > 0` implies the window still fits within the
        // iteration domain `[from, to)`, so `new_start < new_end` and the
        // interval is well-formed.
        let new_interval = unsafe { I::new_unchecked(new_start, new_end) };

        let points = self.stack.change_points();

        // Advance point_start past change points at or before the new start.
        while self.point_start < points.len() && points[self.point_start].at <= new_start {
            self.height_at_start = points[self.point_start].height_after;
            self.point_start += 1;
        }

        // Advance point_end past change points strictly before the new end.
        while self.point_end < points.len() && points[self.point_end].at < new_end {
            self.point_end += 1;
        }

        self.interval = new_interval;
        self.remaining -= 1;
    }
}

impl<'a, I> Iterator for WindowIter<'a, I>
where
    I: IntCO + COStartLenConstruct + Copy,
{
    type Item = StackWindow<'a, I>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let window = StackWindow {
            stack: self.stack,
            interval: self.interval,
            point_start: self.point_start,
            point_end: self.point_end,
            height_at_start: self.height_at_start,
        };
        self.advance();
        Some(window)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'a, I> ExactSizeIterator for WindowIter<'a, I>
where
    I: IntCO + COStartLenConstruct + Copy,
{
    #[inline]
    fn len(&self) -> usize {
        self.remaining
    }
}

#[cfg(test)]
mod tests_for_window_iter;

#[cfg(test)]
mod tests_for_new;

#[cfg(test)]
mod tests_for_height_run_at;

#[cfg(test)]
mod tests_for_iter_height_runs;
