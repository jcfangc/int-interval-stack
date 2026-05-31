use super::*;

impl<I> IntCOStack<I>
where
    I: IntCO,
{
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    #[inline]
    pub fn contains_point(&self, x: I::CoordType) -> bool {
        self.height_at(x) != 0
    }

    pub fn intersects_interval<Q>(&self, query: Q) -> bool
    where
        Q: IntCO<CoordType = I::CoordType>,
    {
        if query.start() >= query.end_excl() {
            return false;
        }

        if self.height_at(query.start()) != 0 {
            return true;
        }

        let i = self.points.partition_point(|p| p.at <= query.start());

        self.points[i..]
            .iter()
            .take_while(|p| p.at < query.end_excl())
            .any(|p| p.height_after != 0)
    }

    pub fn contains_interval<Q>(&self, query: Q) -> bool
    where
        Q: IntCO<CoordType = I::CoordType>,
    {
        if query.start() >= query.end_excl() {
            return true;
        }

        if self.height_at(query.start()) == 0 {
            return false;
        }

        let i = self.points.partition_point(|p| p.at <= query.start());

        !self.points[i..]
            .iter()
            .take_while(|p| p.at < query.end_excl())
            .any(|p| p.height_after == 0)
    }
}
#[cfg(test)]
mod tests_for_predicates;
