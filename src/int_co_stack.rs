use std::sync::Arc;

use int_interval::traits::IntCO;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChangePoint<C> {
    pub at: C,
    pub height_after: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntCOStack<I>
where
    I: IntCO,
{
    points: Arc<[ChangePoint<I::CoordType>]>,
}

mod impls_for_access;
mod impls_for_construction;
mod impls_for_iter;
mod impls_for_predicates;

#[cfg(test)]
pub(crate) mod test_support;
