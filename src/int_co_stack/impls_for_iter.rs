use std::num::NonZeroUsize;

use either::Either;

use super::*;

impl<I> IntCOStack<I>
where
    I: IntCO,
{
    /// Iterates over positive-height segments by scanning canonical change points.
    ///
    /// This is the height-preserving fallback path used when the covered set alone
    /// is not sufficient. Unlike `covered.iter_intervals()`, this iterator keeps
    /// every height boundary, so adjacent covered ranges with different heights are
    /// yielded as separate segments.
    ///
    /// Zero-height gaps are skipped. For each adjacent change-point pair
    /// `(p[i], p[i + 1])`, the segment height is `p[i].height_after`.
    #[inline]
    fn iter_segments_from_change_points(&self) -> impl Iterator<Item = HeightSegment<I>> + '_ {
        self.change_points.windows(2).filter_map(|w| {
            let start = w[0].at;
            let end_excl = w[1].at;

            NonZeroUsize::new(w[0].height_after).map(|height| HeightSegment {
                // SAFETY:
                // Canonical change points are strictly increasing, so every
                // adjacent pair forms a valid non-empty interval.
                interval: unsafe { I::new_unchecked(start, end_excl) },
                height,
            })
        })
    }

    /// Iterates over positive-height stack segments.
    ///
    /// Each item is a closed-open interval together with the stack height that
    /// applies throughout that interval.
    ///
    /// If all positive-height regions have the same height, the segment intervals
    /// are exactly the covered set intervals and the shared height is supplied
    /// from `height_stats`. Otherwise, the height-preserving segmentation is
    /// reconstructed from change points.
    #[inline]
    pub fn iter_height_segments(&self) -> impl Iterator<Item = HeightSegment<I>> + '_ {
        if let Some(height) = self.height_stats.uniform_positive_height() {
            Either::Left(
                self.covered()
                    .iter_intervals()
                    .map(move |interval| HeightSegment { interval, height }),
            )
        } else {
            Either::Right(self.iter_segments_from_change_points())
        }
    }

    /// Iterates over positive-height stack segments whose height is at least
    /// `min_height`.
    ///
    /// Each item is a closed-open interval together with the stack height that
    /// applies throughout that interval.
    ///
    /// This method uses `height_stats` for cheap fast paths:
    ///
    /// - if `min_height` is greater than the observed maximum height, the iterator
    ///   is empty;
    /// - if `min_height` is less than or equal to the observed minimum positive
    ///   height, every positive-height segment matches and `iter_height_segments`
    ///   is reused;
    /// - otherwise, canonical change points are scanned and filtered.
    ///
    /// A `min_height` of zero is treated the same as requesting all positive-height
    /// segments, because zero-height gaps are never yielded.
    #[inline]
    pub fn iter_height_segments_at_least(
        &self,
        min_height: usize,
    ) -> impl Iterator<Item = HeightSegment<I>> + '_ {
        let stack_max = self.height_stats.max_height();

        if min_height > stack_max {
            Either::Left(std::iter::empty())
        } else if min_height <= self.height_stats.min_positive_height_or_zero() {
            Either::Right(Either::Left(self.iter_height_segments()))
        } else {
            Either::Right(Either::Right(
                self.iter_segments_from_change_points()
                    .filter(move |segment| segment.height.get() >= min_height),
            ))
        }
    }

    /// Iterates over positive-height stack segments whose height is at most
    /// `max_height`.
    ///
    /// Each item is a closed-open interval together with the stack height that
    /// applies throughout that interval.
    ///
    /// This method uses `height_stats` for cheap fast paths:
    ///
    /// - if `max_height` is zero, the iterator is empty because zero-height gaps
    ///   are never yielded;
    /// - if `max_height` is smaller than the observed minimum positive height, the
    ///   iterator is empty;
    /// - if `max_height` is greater than or equal to the observed maximum height,
    ///   every positive-height segment matches and `iter_height_segments` is reused;
    /// - otherwise, canonical change points are scanned and filtered.
    #[inline]
    pub fn iter_height_segments_at_most(
        &self,
        max_height: usize,
    ) -> impl Iterator<Item = HeightSegment<I>> + '_ {
        let stack_min = self.height_stats.min_positive_height_or_zero();

        if max_height == 0 || max_height < stack_min {
            Either::Left(std::iter::empty())
        } else if max_height >= self.height_stats.max_height() {
            Either::Right(Either::Left(self.iter_height_segments()))
        } else {
            Either::Right(Either::Right(
                self.iter_segments_from_change_points()
                    .filter(move |segment| segment.height.get() <= max_height),
            ))
        }
    }

    /// Iterates over positive-height stack segments whose height is exactly
    /// `target_height`.
    ///
    /// Each item is a closed-open interval together with the stack height that
    /// applies throughout that interval.
    ///
    /// This method uses `height_stats` for cheap fast paths:
    ///
    /// - if `target_height` is zero, the iterator is empty because zero-height gaps
    ///   are never yielded;
    /// - if `target_height` is outside the observed positive-height range, the
    ///   iterator is empty;
    /// - if all positive-height regions share the same height and `target_height`
    ///   equals that height, the intervals are exactly the covered set intervals;
    /// - otherwise, canonical change points are scanned and filtered.
    #[inline]
    pub fn iter_height_segments_exactly(
        &self,
        target_height: usize,
    ) -> impl Iterator<Item = HeightSegment<I>> + '_ {
        let Some(target_height) = NonZeroUsize::new(target_height) else {
            return Either::Left(std::iter::empty());
        };

        let target = target_height.get();

        if target < self.height_stats.min_positive_height_or_zero()
            || target > self.height_stats.max_height()
        {
            Either::Left(std::iter::empty())
        } else if self.height_stats.uniform_positive_height() == Some(target_height) {
            Either::Right(Either::Left(self.covered().iter_intervals().map(
                move |interval| HeightSegment {
                    interval,
                    height: target_height,
                },
            )))
        } else {
            Either::Right(Either::Right(
                self.iter_segments_from_change_points()
                    .filter(move |segment| segment.height == target_height),
            ))
        }
    }

    /// Iterates over positive-height stack segments whose height is within
    /// `min_height..=max_height`.
    ///
    /// Each item is a closed-open interval together with the stack height that
    /// applies throughout that interval.
    ///
    /// Zero-height gaps are never yielded. Therefore, a `min_height` of zero is
    /// treated as if it included all positive heights.
    ///
    /// This method uses `height_stats` for cheap fast paths:
    ///
    /// - if `min_height > max_height`, the iterator is empty;
    /// - if the stack has no positive-height segments, the iterator is empty;
    /// - if the requested range does not overlap the observed positive-height
    ///   range, the iterator is empty;
    /// - if the requested range covers the full observed positive-height range,
    ///   every segment matches and `iter_height_segments` is reused;
    /// - otherwise, canonical change points are scanned and filtered.
    #[inline]
    pub fn iter_height_segments_between(
        &self,
        min_height: usize,
        max_height: usize,
    ) -> impl Iterator<Item = HeightSegment<I>> + '_ {
        let stack_min = self.height_stats.min_positive_height_or_zero();
        let stack_max = self.height_stats.max_height();
        let query_min = min_height.max(1);

        if min_height > max_height
            || !self.height_stats.has_positive_height()
            || max_height < stack_min
            || query_min > stack_max
        {
            Either::Left(std::iter::empty())
        } else if query_min <= stack_min && max_height >= stack_max {
            Either::Right(Either::Left(self.iter_height_segments()))
        } else {
            Either::Right(Either::Right(
                self.iter_segments_from_change_points()
                    .filter(move |segment| {
                        query_min <= segment.height.get() && segment.height.get() <= max_height
                    }),
            ))
        }
    }

    /// Iterates over positive-height stack segments whose height is the observed
    /// maximum stack height.
    ///
    /// Each item is a closed-open interval together with the peak height that
    /// applies throughout that interval.
    ///
    /// If the stack has no positive-height segments, the iterator is empty.
    ///
    /// This is equivalent to:
    ///
    /// ```text
    /// iter_height_segments_exactly(height_stats.max_height())
    /// ```
    #[inline]
    pub fn iter_peak_height_segments(&self) -> impl Iterator<Item = HeightSegment<I>> + '_ {
        self.iter_height_segments_exactly(self.height_stats.max_height())
    }
}

#[cfg(test)]
mod tests_for_iter_segments_from_change_points;

#[cfg(test)]
mod tests_for_iter_height_segments;
