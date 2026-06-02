use super::*;

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
        &self.covered
    }

    #[inline]
    pub fn height_stats(&self) -> StackHeightStats {
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
