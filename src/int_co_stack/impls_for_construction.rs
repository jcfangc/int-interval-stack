use super::*;

use rayon::iter::{FromParallelIterator, IntoParallelIterator, ParallelIterator};

const BATCH_SIZE: usize = 128;

impl<I> Default for IntCOStack<I>
where
    I: IntCO,
{
    #[inline]
    fn default() -> Self {
        Self {
            points: Arc::from([]),
            height_stats: StackHeightStats::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EndpointKind {
    Enter,
    Leave,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Endpoint<C> {
    at: C,
    kind: EndpointKind,
}

#[derive(Debug, Default)]
struct StackParts<C>
where
    C: Default,
{
    points: Vec<ChangePoint<C>>,
    height_stats: StackHeightStats,
}

/// Builds a canonical stack-height function from raw interval endpoint events.
///
/// Each half-open interval contributes two endpoint events:
///
/// ```text
/// [start, end) -> Enter at start, Leave at end
/// ```
///
/// Events at the same coordinate are applied together because a half-open
/// boundary may contain both intervals ending and intervals starting. Only
/// the resulting net height matters for the canonical representation.
///
/// The returned change points describe a piecewise-constant height function:
/// after each `at`, the active interval count becomes `height_after`.
///
/// # Input assumptions
///
/// The endpoint events must originate from valid finite half-open intervals.
/// Consequently:
///
/// - the accumulated height never becomes negative;
/// - after all events are consumed, the accumulated height returns to zero.
///
/// # Canonical output
///
/// The returned points satisfy:
///
/// - coordinates are strictly increasing;
/// - adjacent points always have different `height_after` values;
/// - redundant coordinates whose events cause no net height change are omitted.
///
/// # Complexity
///
/// Sorting dominates the computation: `O(n log n)` time for `n` endpoints.
/// The output allocates at most `n` change points.
fn build_parts_from_endpoints<C>(mut endpoints: Vec<Endpoint<C>>) -> StackParts<C>
where
    C: Default + Copy + Ord,
{
    // Ordered endpoints allow all events at the same coordinate to be
    // consumed together and emitted as at most one canonical change point.
    endpoints.sort_unstable_by_key(|endpoint| endpoint.at);

    let mut points = Vec::with_capacity(endpoints.len());
    let mut height_stats = StackHeightStats::default();

    // Height of the piecewise-constant function immediately before the next
    // unprocessed coordinate. The height before the first endpoint is zero.
    let mut height_after = 0usize;
    let mut cursor = 0usize;

    while cursor < endpoints.len() {
        let at = endpoints[cursor].at;
        let mut enters = 0usize;
        let mut leaves = 0usize;

        // Apply every boundary event at `at` atomically. For half-open
        // intervals, intervals leaving at `at` are already inactive there,
        // while intervals entering at `at` are active from there onward.
        while cursor < endpoints.len() && endpoints[cursor].at == at {
            match endpoints[cursor].kind {
                EndpointKind::Enter => enters += 1,
                EndpointKind::Leave => leaves += 1,
            }
            cursor += 1;
        }

        // Compute the height on the segment beginning at `at`. The split
        // between addition and subtraction keeps the stored height unsigned
        // while still detecting malformed events that would go below zero.
        let next_height = if enters >= leaves {
            height_after.checked_add(enters - leaves)
        } else {
            height_after.checked_sub(leaves - enters)
        }
        .expect("valid intervals must never produce a negative stack height");

        height_stats.observe(next_height);

        // Emit only real changes of the height function. Equal numbers of
        // entering and leaving layers at the same coordinate cancel out and
        // must not create a redundant canonical boundary.
        if next_height != height_after {
            points.push(ChangePoint {
                at,
                height_after: next_height,
            });
        }

        height_after = next_height;
    }

    // Every finite half-open interval contributes one enter and one leave, so
    // a complete valid event stream must end outside all intervals.
    debug_assert_eq!(
        height_after, 0,
        "all finite half-open intervals must eventually close"
    );

    StackParts {
        points,
        height_stats,
    }
}

/// Merges two canonical change-point sequences by adding their stack heights.
///
/// Each input slice represents a piecewise-constant stack-height function:
/// after a change point at `at`, the function takes the value
/// `height_after` until the next change point.
///
/// The merged sequence represents the pointwise sum of the two functions:
///
/// ```text
/// merged_height(x) = lhs_height(x) + rhs_height(x)
/// ```
///
/// Change points that would not change the merged height are omitted. This
/// preserves the canonical representation, including cases where a boundary
/// in one input is exactly cancelled by a boundary in the other input.
///
/// # Input invariants
///
/// Both input slices must be canonical:
///
/// - change points are ordered by strictly increasing coordinates;
/// - adjacent change points have different `height_after` values;
/// - the height before the first change point is zero;
/// - the final change point, when present, restores the height to zero.
///
/// # Panics
///
/// Panics if the sum of the two active heights exceeds [`usize::MAX`].
///
/// # Complexity
///
/// Runs in `O(lhs.len() + rhs.len())` time and allocates at most
/// `lhs.len() + rhs.len()` output change points.
fn merge_parts<C>(lhs: &StackParts<C>, rhs: &StackParts<C>) -> StackParts<C>
where
    C: Default + Copy + Ord,
{
    let lhs_points_len = lhs.points.len();
    let rhs_points_len = rhs.points.len();

    let mut out_points = Vec::with_capacity(lhs_points_len + rhs_points_len);
    let mut out_stats = StackHeightStats::default();

    let mut lhs_height = 0usize;
    let mut rhs_height = 0usize;
    let mut merged_height = 0usize;

    let mut lhs_cursor = 0usize;
    let mut rhs_cursor = 0usize;

    while lhs_cursor < lhs_points_len || rhs_cursor < rhs_points_len {
        let at = match (lhs.points.get(lhs_cursor), rhs.points.get(rhs_cursor)) {
            // Only `lhs` changes at this coordinate; keep the current
            // height contributed by `rhs`.
            (Some(l), Some(r)) if l.at < r.at => {
                lhs_height = l.height_after;
                lhs_cursor += 1;
                l.at
            }

            // Only `rhs` changes at this coordinate; keep the current
            // height contributed by `lhs`.
            (Some(l), Some(r)) if r.at < l.at => {
                rhs_height = r.height_after;
                rhs_cursor += 1;
                r.at
            }

            // Both functions change at the same coordinate. Apply both
            // changes before computing the merged height after `at`.
            (Some(l), Some(r)) => {
                lhs_height = l.height_after;
                rhs_height = r.height_after;
                lhs_cursor += 1;
                rhs_cursor += 1;
                l.at
            }

            // `rhs` has been exhausted; append the remaining changes from
            // `lhs` while preserving the final height contributed by `rhs`.
            (Some(l), None) => {
                lhs_height = l.height_after;
                lhs_cursor += 1;
                l.at
            }

            // `lhs` has been exhausted; append the remaining changes from
            // `rhs` while preserving the final height contributed by `lhs`.
            (None, Some(r)) => {
                rhs_height = r.height_after;
                rhs_cursor += 1;
                r.at
            }

            // The loop condition guarantees that at least one input still
            // contains an unprocessed change point.
            (None, None) => unreachable!(),
        };

        let next_merged_height = lhs_height
            .checked_add(rhs_height)
            .expect("stack height overflow");

        out_stats.observe(next_merged_height);

        // A coordinate belongs to the canonical output exactly when the
        // pointwise sum changes its value at that coordinate.
        if next_merged_height != merged_height {
            out_points.push(ChangePoint {
                at,
                height_after: next_merged_height,
            });
            merged_height = next_merged_height;
        }
    }

    debug_assert_eq!(merged_height, 0);

    StackParts {
        points: out_points,
        height_stats: out_stats,
    }
}

#[derive(Debug)]
struct StackBuildAcc<C>
where
    C: Default + Copy + Ord,
{
    endpoints: Vec<Endpoint<C>>,
    levels: Vec<Option<StackParts<C>>>,
}

impl<C> StackBuildAcc<C>
where
    C: Default + Copy + Ord,
{
    #[inline]
    fn new() -> Self {
        Self {
            // Buffer one bounded batch of raw interval boundary events.
            //
            // Each interval contributes two endpoints, so the endpoint
            // capacity is twice the interval batch size.
            endpoints: Vec::with_capacity(BATCH_SIZE.saturating_mul(2)),

            // Balanced partial canonical change-point sequences.
            //
            // This acts like a binary carry chain: inserting into an occupied
            // level merges two equal-rank partial results and carries the
            // merged sequence upward.
            levels: Vec::new(),
        }
    }

    #[inline]
    fn push_interval<I>(&mut self, interval: I)
    where
        I: IntCO<CoordType = C>,
    {
        // Convert one half-open interval into raw boundary events:
        // [start, end) -> Enter at start, Leave at end.
        self.endpoints.push(Endpoint {
            at: interval.start(),
            kind: EndpointKind::Enter,
        });
        self.endpoints.push(Endpoint {
            at: interval.end_excl(),
            kind: EndpointKind::Leave,
        });

        // Once the local endpoint buffer reaches one full batch, compact it
        // into canonical change points and insert it into the level chain.
        if self.endpoints.len() >= BATCH_SIZE.saturating_mul(2) {
            self.flush();
        }
    }

    #[inline]
    fn finish(mut self) -> StackParts<C> {
        // Make sure the final partial batch is included.
        self.flush();

        // Merge all remaining partial sequences into one canonical stack.
        // Empty input naturally produces an empty change-point sequence.
        self.levels
            .into_iter()
            .flatten()
            .reduce(|lhs, rhs| merge_parts(&lhs, &rhs))
            .unwrap_or_default()
    }

    #[inline]
    fn flush(&mut self) {
        if self.endpoints.is_empty() {
            return;
        }

        // Move out the current endpoint batch while leaving a fresh buffer for
        // subsequent pushes. The old buffer is consumed by sorting and
        // canonicalization.
        let endpoints = core::mem::replace(
            &mut self.endpoints,
            Vec::with_capacity(BATCH_SIZE.saturating_mul(2)),
        );

        // Canonicalize this endpoint batch, then insert the resulting
        // stack-height function into the balanced level chain.
        self.push_points(build_parts_from_endpoints(endpoints));
    }

    fn push_points(&mut self, mut carry: StackParts<C>) {
        let mut level = 0usize;

        loop {
            if level == self.levels.len() {
                // No level exists yet, so append a new top level.
                self.levels.push(Some(carry));
                break;
            }

            match self.levels[level].take() {
                // Empty level: store the current carry here.
                None => {
                    self.levels[level] = Some(carry);
                    break;
                }

                // Occupied level: merge two equal-rank canonical sequences and
                // carry the combined result into the next level.
                Some(parts) => {
                    carry = merge_parts(&parts, &carry);
                    level += 1;
                }
            }
        }
    }
}

impl<I> FromIterator<I> for IntCOStack<I>
where
    I: IntCO,
{
    #[inline]
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = I>,
    {
        let mut acc = StackBuildAcc::new();

        for interval in iter {
            acc.push_interval(interval);
        }

        let StackParts {
            points,
            height_stats,
        } = acc.finish();

        Self {
            points: points.into(),
            height_stats,
        }
    }
}

impl<I> FromParallelIterator<I> for IntCOStack<I>
where
    I: IntCO + Send,
{
    #[inline]
    fn from_par_iter<T>(par_iter: T) -> Self
    where
        T: IntoParallelIterator<Item = I>,
    {
        let parts = par_iter
            .into_par_iter()
            // Each Rayon worker builds a local canonical stack from its own
            // input shard. This keeps endpoint sorting and batch compaction
            // embarrassingly parallel.
            .fold(StackBuildAcc::new, |mut acc, interval| {
                acc.push_interval(interval);
                acc
            })
            // Finalize each local builder before global reduction.
            .map(StackBuildAcc::finish)
            // Merge partial stack-height functions by pointwise addition.
            .reduce(StackParts::default, |lhs, rhs| merge_parts(&lhs, &rhs));

        let StackParts {
            points,
            height_stats,
        } = parts;

        Self {
            points: points.into(),
            height_stats,
        }
    }
}

#[cfg(test)]
pub(crate) mod test_support;

#[cfg(test)]
mod tests_for_build_parts_from_endpoints;

#[cfg(test)]
mod tests_for_merge_parts;

#[cfg(test)]
mod tests_for_stack_build_acc;

#[cfg(test)]
mod tests_for_from_iter_and_from_par_iter;
