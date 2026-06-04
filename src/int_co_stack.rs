use super::*;

use std::sync::Arc;

use int_interval_set::IntCOSet;

use crate::{ChangePoint, HeightStats};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntCOStack<I>
where
    I: IntCO,
{
    change_points: Arc<[ChangePoint<I::CoordType>]>,
    covered: IntCOSet<I>,
    height_stats: HeightStats,
}

mod impls_for_access;
mod impls_for_construction;
mod impls_for_iter;

#[cfg(test)]
pub(crate) mod test_support;
