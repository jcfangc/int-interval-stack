use std::sync::Arc;

use int_interval::traits::IntCO;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChangePoint<C> {
    pub at: C,
    pub height_after: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StackHeightStats {
    min_positive_height_or_zero: usize,
    max_height: usize,
}

impl Default for StackHeightStats {
    fn default() -> Self {
        Self {
            min_positive_height_or_zero: 0,
            max_height: 0,
        }
    }
}

impl StackHeightStats {
    #[inline]
    fn observe(&mut self, h: usize) {
        self.max_height = self.max_height.max(h);

        if h == 0 {
            return;
        }

        if self.min_positive_height_or_zero == 0 || h < self.min_positive_height_or_zero {
            self.min_positive_height_or_zero = h;
        }
    }
}

impl StackHeightStats {
    #[inline]
    pub const fn min_positive_height_or_zero(&self) -> usize {
        self.min_positive_height_or_zero
    }

    #[inline]
    pub const fn max_height(&self) -> usize {
        self.max_height
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntCOStack<I>
where
    I: IntCO,
{
    points: Arc<[ChangePoint<I::CoordType>]>,
    height_stats: StackHeightStats,
}

mod impls_for_access;
mod impls_for_construction;
mod impls_for_iter;
mod impls_for_predicates;

#[cfg(test)]
pub(crate) mod test_support;
