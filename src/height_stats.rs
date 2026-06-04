#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeightStats {
    min_positive_height_or_zero: usize,
    max_height: usize,
}

impl Default for HeightStats {
    fn default() -> Self {
        Self {
            min_positive_height_or_zero: 0,
            max_height: 0,
        }
    }
}

impl HeightStats {
    #[inline]
    pub(crate) fn observe(&mut self, h: usize) {
        self.max_height = self.max_height.max(h);

        if h == 0 {
            return;
        }

        if self.min_positive_height_or_zero == 0 || h < self.min_positive_height_or_zero {
            self.min_positive_height_or_zero = h;
        }
    }
}

impl HeightStats {
    #[inline]
    pub const fn min_positive_height_or_zero(&self) -> usize {
        self.min_positive_height_or_zero
    }

    #[inline]
    pub const fn max_height(&self) -> usize {
        self.max_height
    }
}

impl HeightStats {
    /// Returns whether any positive stack height was observed.
    #[inline]
    pub const fn has_positive_height(&self) -> bool {
        self.max_height != 0
    }

    /// Returns whether at least one coordinate range is covered by multiple
    /// intervals.
    #[inline]
    pub const fn has_overlap(&self) -> bool {
        self.max_height > 1
    }

    /// Returns whether all positive-height regions share the same height.
    #[inline]
    pub const fn is_uniform_positive_height(&self) -> bool {
        self.min_positive_height_or_zero != 0 && self.min_positive_height_or_zero == self.max_height
    }
}

#[cfg(test)]
pub(crate) mod test_support;
