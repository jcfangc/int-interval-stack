use int_interval::I8CO;

use super::*;

pub(super) fn window_bounds(window: StackWindow<'_, I8CO>) -> (i8, i8) {
    (window.interval().start(), window.interval().end_excl())
}
