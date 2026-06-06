use int_interval::I8CO;

use super::*;

pub(super) fn window_bounds(window: StackWindow<'_, I8CO>) -> (i8, i8) {
    (window.interval().start(), window.interval().end_excl())
}

fn iv_i8(start: i8, end_excl: i8) -> I8CO {
    I8CO::try_new(start, end_excl).unwrap()
}

pub(super) fn stack_from_i8_intervals(intervals: &[(i8, i8)]) -> IntCOStack<I8CO> {
    intervals
        .iter()
        .copied()
        .map(|(start, end_excl)| iv_i8(start, end_excl))
        .collect()
}

pub(super) fn run_bounds(window: StackWindow<'_, I8CO>) -> Vec<((i8, i8), usize)> {
    window
        .iter_height_runs()
        .map(|run| ((run.interval.start(), run.interval.end_excl()), run.height))
        .collect()
}
