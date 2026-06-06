use super::*;

/// Projects the covered interval set from canonical stack change points.
///
/// A stack change-point sequence describes a piecewise-constant height
/// function. The covered set is exactly the union of all coordinate ranges
/// whose height is positive.
///
/// This function scans positive-height runs:
///
/// ```text
/// height: 0 -> positive    opens a covered interval
/// height: positive -> 0    closes a covered interval
/// positive -> positive     keeps the current covered interval open
/// ```
///
/// # Input invariants
///
/// `points` must be the canonical output of stack construction:
///
/// - coordinates are strictly increasing;
/// - adjacent points have different `height_after` values;
/// - the final height, when non-empty, is zero.
///
/// # Output
///
/// The returned set is canonical. Positive-height runs are emitted in
/// ascending order and are separated by zero-height gaps.
#[inline]
fn covered_from_change_points<I>(points: &[ChangePoint<I::CoordType>]) -> IntCOSet<I>
where
    I: IntCO,
{
    let mut out = Vec::new();
    let mut start = None;

    for p in points {
        match (start, p.height_after) {
            (None, h) if h > 0 => {
                start = Some(p.at);
            }
            (Some(s), 0) => {
                // SAFETY:
                // Canonical change points are strictly increasing, and a
                // positive-height run can only close at a later coordinate.
                out.push(unsafe { I::new_unchecked(s, p.at) });
                start = None;
            }
            _ => {}
        }
    }

    debug_assert!(
        start.is_none(),
        "canonical stack change points must end at zero height"
    );

    // SAFETY:
    // Positive-height runs are emitted in ascending order and are separated by
    // zero-height gaps, so the result is canonical.
    unsafe { IntCOSet::new_unchecked(out) }
}

impl<I> IntCOStack<I>
where
    I: IntCO,
{
    #[inline]
    pub fn change_points(&self) -> &[ChangePoint<I::CoordType>] {
        &self.change_points
    }

    #[inline]
    pub fn covered(&self) -> &IntCOSet<I> {
        self.covered
            .get_or_init(|| covered_from_change_points::<I>(&self.change_points))
    }

    #[inline]
    pub fn height_stats(&self) -> HeightStats {
        self.height_stats
    }

    #[inline]
    pub fn height_at(&self, x: I::CoordType) -> usize {
        let i = self.change_points.partition_point(|p| p.at <= x);
        if i == 0 {
            0
        } else {
            self.change_points[i - 1].height_after
        }
    }
}

#[cfg(test)]
mod tests_for_access;

#[cfg(test)]
mod tests_for_covered_from_change_points;
