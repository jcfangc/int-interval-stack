use crate::ChangePoint;

use super::*;

pub(crate) fn height_stats_from_points<C>(points: &[ChangePoint<C>]) -> HeightStats {
    let mut stats = HeightStats::default();

    for point in points {
        stats.observe(point.height_after);
    }

    stats
}
