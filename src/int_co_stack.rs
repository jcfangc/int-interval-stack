use super::*;

use std::sync::{Arc, OnceLock};

use int_interval_set::IntCOSet;

use crate::{ChangePoint, HeightStats};

#[derive(Debug)]
pub struct IntCOStack<I>
where
    I: IntCO,
{
    change_points: Arc<[ChangePoint<I::CoordType>]>,
    height_stats: HeightStats,
    covered: OnceLock<IntCOSet<I>>,
}

mod impls_for_access;
mod impls_for_construction;
mod impls_for_derived_traits;
mod impls_for_iter;
mod impls_for_windows;

#[cfg(test)]
pub(crate) mod test_support;
