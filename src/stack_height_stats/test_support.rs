use crate::ChangePoint;

use super::*;

pub(crate) fn height_stats_from_points<C>(points: &[ChangePoint<C>]) -> StackHeightStats {
    let mut stats = StackHeightStats::default();

    for point in points {
        stats.observe(point.height_after);
    }

    stats
}
