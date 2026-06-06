use int_interval::traits::IntCO;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use crate::{HeightRun, IntCOStack};

#[derive(Debug, Clone, Copy)]
pub struct StackWindow<'a, I>
where
    I: IntCO,
{
    stack: &'a IntCOStack<I>,
    interval: I,
    /// First change point strictly inside the window after `interval.start()`.
    point_start: usize,
    /// First change point at or after `interval.end_excl()`.
    point_end: usize,
    /// Stack height at `interval.start()`.
    height_at_start: usize,
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

#[cfg(test)]
mod tests_for_new;

#[cfg(test)]
mod tests_for_height_run_at;

#[cfg(test)]
mod tests_for_iter_height_runs;
