use super::*;

impl<I> IntCOStack<I>
where
    I: IntCO,
{
    #[inline]
    pub fn change_points(&self) -> &[ChangePoint<I::CoordType>] {
        &self.points
    }

    pub fn height_at(&self, x: I::CoordType) -> usize {
        let i = self.points.partition_point(|p| p.at <= x);
        if i == 0 {
            0
        } else {
            self.points[i - 1].height_after
        }
    }

    #[inline]
    pub fn max_height(&self) -> usize {
        self.points
            .iter()
            .map(|p| p.height_after)
            .max()
            .unwrap_or(0)
    }
}
