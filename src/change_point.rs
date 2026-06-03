#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChangePoint<C> {
    pub at: C,
    pub height_after: usize,
}

#[cfg(test)]
pub(crate) mod test_support;
