use super::*;

impl<I> Clone for IntCOStack<I>
where
    I: IntCO,
{
    fn clone(&self) -> Self {
        Self {
            change_points: self.change_points.clone(),
            height_stats: self.height_stats,
            covered: OnceLock::new(),
        }
    }
}

impl<I> PartialEq for IntCOStack<I>
where
    I: IntCO,
{
    fn eq(&self, other: &Self) -> bool {
        self.change_points == other.change_points && self.height_stats == other.height_stats
    }
}

impl<I> Eq for IntCOStack<I> where I: IntCO {}

#[cfg(test)]
mod tests_for_clone_and_eq;
