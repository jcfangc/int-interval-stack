use super::*;

impl<I> IntCOStack<I>
where
    I: IntCO,
{
    #[inline]
    pub fn iter_intervals(&self) -> impl Iterator<Item = (I, usize)> {
        self.points.windows(2).filter_map(|w| {
            let start = w[0].at;
            let end_excl = w[1].at;
            let height = w[0].height_after;

            (height != 0).then_some((unsafe { I::new_unchecked(start, end_excl) }, height))
        })
    }

    #[inline]
    pub fn iter_intervals_at_least(&self, min_height: usize) -> impl Iterator<Item = (I, usize)> {
        self.points.windows(2).filter_map(move |w| {
            let start = w[0].at;
            let end_excl = w[1].at;
            let height = w[0].height_after;

            (height != 0 && height >= min_height)
                .then_some((unsafe { I::new_unchecked(start, end_excl) }, height))
        })
    }
    #[inline]
    pub fn iter_intervals_at_most(&self, max_height: usize) -> impl Iterator<Item = (I, usize)> {
        self.points.windows(2).filter_map(move |w| {
            let start = w[0].at;
            let end_excl = w[1].at;
            let height = w[0].height_after;

            (height != 0 && height <= max_height)
                .then_some((unsafe { I::new_unchecked(start, end_excl) }, height))
        })
    }

    #[inline]
    pub fn iter_intervals_exactly(&self, target_height: usize) -> impl Iterator<Item = (I, usize)> {
        self.points.windows(2).filter_map(move |w| {
            let start = w[0].at;
            let end_excl = w[1].at;
            let height = w[0].height_after;

            (height != 0 && height == target_height)
                .then_some((unsafe { I::new_unchecked(start, end_excl) }, height))
        })
    }

    #[inline]
    pub fn iter_intervals_between(
        &self,
        min_height: usize,
        max_height: usize,
    ) -> impl Iterator<Item = (I, usize)> {
        self.points.windows(2).filter_map(move |w| {
            let start = w[0].at;
            let end_excl = w[1].at;
            let height = w[0].height_after;

            (height != 0 && height >= min_height && height <= max_height)
                .then_some((unsafe { I::new_unchecked(start, end_excl) }, height))
        })
    }

    #[inline]
    pub fn peak_intervals(&self) -> impl Iterator<Item = (I, usize)> {
        let max_height = self.max_height();

        self.iter_intervals()
            .filter(move |(_, height)| *height == max_height)
    }
}
