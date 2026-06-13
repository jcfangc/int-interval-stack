use int_interval::traits::IntCO;
use int_interval::{I8CO, I32CO};
use std::num::NonZeroUsize;

use crate::IntCOStack;
use crate::stack_window::WindowCursor;

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

#[inline]
fn iv(start: i32, end_excl: i32) -> I32CO {
    I32CO::try_new(start, end_excl).unwrap()
}

#[inline]
fn stack_from_intervals(intervals: &[(i32, i32)]) -> IntCOStack<I32CO> {
    intervals.iter().copied().map(|(s, e)| iv(s, e)).collect()
}

fn cursor_fields<I>(cursor: &WindowCursor<'_, I>) -> (usize, usize, usize)
where
    I: IntCO,
{
    (cursor.point_start, cursor.point_end, cursor.height_at_start)
}

// ---------------------------------------------------------------------------
// Cursor initialization
// ---------------------------------------------------------------------------

#[test]
fn cursor_init_matches_stack_window_new() {
    let stack = stack_from_intervals(&[(1, 4), (3, 6)]);

    // First window [0, 3)
    let interval = I32CO::checked_from_start_len(0, 3u32).unwrap();
    let sw = crate::StackWindow::new(&stack, interval);

    let cursor = WindowCursor::new(
        &stack,
        0i32,
        3u32,
        NonZeroUsize::new(5).unwrap(), // windows: [0,3),[1,4),[2,5),[3,6),[4,7)
    );

    assert_eq!(cursor.point_start, sw.point_start);
    assert_eq!(cursor.point_end, sw.point_end);
    assert_eq!(cursor.height_at_start, sw.height_at_start);
}

#[test]
fn cursor_init_window_before_first_change_point() {
    // Stack: [(2, 5)]
    let stack = stack_from_intervals(&[(2, 5)]);

    // Window [0, 2) — entirely before first change point
    let cursor = WindowCursor::new(
        &stack,
        0i32,
        2u32,
        NonZeroUsize::new(3).unwrap(), // [0,2), [1,3), [2,4)
    );

    // point_start = 0 (no CP at or before 0, but partition_point of [] returns 0)
    // Actually: points = [(2,1), (5,0)]. partition_point(|p| p.at <= 0) = 0
    // So point_start = 0, no previous CP, height_at_start = 0
    assert_eq!(cursor_fields(&cursor), (0, 0, 0));

    // Verify the window content matches
    let window = cursor.window();
    assert_eq!(window.interval().start(), 0);
    assert_eq!(window.interval().end_excl(), 2);

    let runs: Vec<_> = window.iter_height_runs().collect();
    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].height, 0);
}

#[test]
fn cursor_init_window_with_multiple_interior_change_points() {
    // Stack: [(1, 4), (3, 6)]
    // points: [(1,1), (3,2), (4,1), (6,0)]
    let stack = stack_from_intervals(&[(1, 4), (3, 6)]);

    // Window [2, 5):
    // point_start: first CP at > 2 → CP(3,2) at index 1
    // point_end: first CP at >= 5 → CP(6,0) at index 3
    let interval = I32CO::checked_from_start_len(2, 3u32).unwrap();
    let sw = crate::StackWindow::new(&stack, interval);
    assert_eq!(
        (sw.point_start, sw.point_end, sw.height_at_start),
        (1, 3, 1)
    );

    let cursor = WindowCursor::new(
        &stack,
        2i32,
        3u32,
        NonZeroUsize::new(2).unwrap(), // [2,5), [3,6)
    );
    assert_eq!(cursor_fields(&cursor), (1, 3, 1));
}

#[test]
fn cursor_init_empty_stack() {
    let stack: IntCOStack<I32CO> = IntCOStack::default();

    let cursor = WindowCursor::new(&stack, 0i32, 3u32, NonZeroUsize::new(2).unwrap());

    // No change points at all — everything is zero
    assert_eq!(cursor_fields(&cursor), (0, 0, 0));

    let window = cursor.window();
    let runs: Vec<_> = window.iter_height_runs().collect();
    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].height, 0);
}

// ---------------------------------------------------------------------------
// Cursor advance
// ---------------------------------------------------------------------------

#[test]
fn cursor_advance_indices_incrementally() {
    // Stack: [(1, 4), (3, 6)] → points: [(1,1), (3,2), (4,1), (6,0)]
    let stack = stack_from_intervals(&[(1, 4), (3, 6)]);

    // windows: [1,4), [2,5), [3,6), [4,7)
    let mut cursor = WindowCursor::new(&stack, 1i32, 3u32, NonZeroUsize::new(4).unwrap());

    // Window [1,4): start=1, end=4
    // point_start: partition_point ≤ 1 → points[0]=(1,1) has at=1≤1 → skip it → index 1
    // point_end from pt_start=1: points[1]=(3,2) at=3<4 → skip → index 2. points[2]=(4,1) at=4<4? No → stop
    // So: (1, 2, ?)
    // height_at_start: point_start=1, points[0].height_after=1
    assert_eq!(cursor_fields(&cursor), (1, 2, 1));

    // Verify window [1,4) from cursor matches StackWindow::new
    {
        let interval = I32CO::checked_from_start_len(1, 3u32).unwrap();
        let sw = crate::StackWindow::new(&stack, interval);
        let cw = cursor.window();
        assert_eq!(cw.interval().start(), sw.interval().start());
        assert_eq!(cw.interval().end_excl(), sw.interval().end_excl());
        let sw_runs: Vec<_> = sw.iter_height_runs().collect();
        let cw_runs: Vec<_> = cw.iter_height_runs().collect();
        assert_eq!(cw_runs, sw_runs);
    }

    // Advance to [2,5)
    cursor.advance();
    // start=2, end=5
    // point_start: points[1]=(3,2) at=3≤2? No. Stays at 1.
    // point_end: points[2]=(4,1) at=4<5 → advance → index 3. points[3]=(6,0) at=6<5? No → stop
    assert_eq!(cursor_fields(&cursor), (1, 3, 1));

    // Verify [2,5) matches StackWindow::new
    {
        let interval = I32CO::checked_from_start_len(2, 3u32).unwrap();
        let sw = crate::StackWindow::new(&stack, interval);
        let cw = cursor.window();
        let sw_runs: Vec<_> = sw.iter_height_runs().collect();
        let cw_runs: Vec<_> = cw.iter_height_runs().collect();
        assert_eq!(cw_runs, sw_runs);
    }

    // Advance to [3,6)
    cursor.advance();
    // start=3, end=6
    // point_start: points[1]=(3,2) at=3≤3 → advance, height=2 → index 2
    //   points[2]=(4,1) at=4≤3? No → stop
    // point_end: points[3]=(6,0) at=6<6? No → stop
    assert_eq!(cursor_fields(&cursor), (2, 3, 2));

    // Advance to [4,7)
    cursor.advance();
    // start=4, end=7
    // point_start: points[2]=(4,1) at=4≤4 → advance, height=1 → index 3
    //   points[3]=(6,0) at=6≤4? No → stop
    // point_end: points[3]=(6,0) at=6<7 → advance → index 4. Done.
    assert_eq!(cursor_fields(&cursor), (3, 4, 1));
    assert_eq!(cursor.remaining, 1);
}

#[test]
fn cursor_advance_past_point_end_only_when_necessary() {
    // Stack: [(0, 10)] → points: [(0,1), (10,0)]
    let stack = stack_from_intervals(&[(0, 10)]);

    // windows of len=2: [0,2), [1,3), ..., [8,10)
    let mut cursor = WindowCursor::new(&stack, 0i32, 2u32, NonZeroUsize::new(9).unwrap());

    // [0,2): start=0, end=2
    // point_start: CP(0,1) at=0≤0 → skip → 1. CP(10,0) at=10≤0? No → index 1
    // point_end from 1: CP(10,0) at=10<2? No → index 1
    // height_at_start: points[0].height_after = 1
    assert_eq!(cursor_fields(&cursor), (1, 1, 1));

    // Slide to [1,3): nothing changes
    cursor.advance();
    assert_eq!(cursor_fields(&cursor), (1, 1, 1));

    // Slide to [2,4): nothing changes
    cursor.advance();
    assert_eq!(cursor_fields(&cursor), (1, 1, 1));

    // Continue sliding... point_start and point_end stay at 1 until start passes CP(0,1)
    // (already past) and end reaches CP(10,0).
    // At [8,10): end=10. point_end: CP(10,0) at=10<10? No → still 1
    // Slide to [9,11): end=11. point_end: CP(10,0) at=10<11 → advance → 2
    for _ in 0..7 {
        cursor.advance();
    }
    // After 7 more slides from [2,4): [9,11)
    assert_eq!(cursor_fields(&cursor), (1, 2, 1));
}

// ---------------------------------------------------------------------------
// Cursor → StackWindow integrity
// ---------------------------------------------------------------------------

#[test]
fn cursor_window_matches_stack_window_new_for_various_stacks() {
    let cases: &[&[(i32, i32)]] = &[
        &[],                          // empty
        &[(0, 10)],                   // single interval
        &[(1, 4), (3, 6)],            // overlapping
        &[(0, 2), (4, 6)],            // disjoint
        &[(0, 10), (2, 8), (4, 6)],   // nested
        &[(0, 4), (4, 8)],            // adjacent
        &[(0, 5), (5, 10), (10, 15)], // chain
    ];

    for intervals in cases {
        let stack = stack_from_intervals(intervals);

        // Use iter_windows to get cursor-based windows
        let cursor_windows: Vec<_> = stack
            .iter_windows(-5, 20, 3u32)
            .map(|w| {
                let runs: Vec<_> = w.iter_height_runs().collect();
                ((w.interval().start(), w.interval().end_excl()), runs)
            })
            .collect();

        // Use StackWindow::new directly for each window (oracle)
        // For domain [-5, 20), len=3: starts at -5 through 17 (23 windows)
        let oracle_windows: Vec<_> = (-5..=17)
            .filter_map(|start| {
                let interval = I32CO::checked_from_start_len(start, 3u32)?;
                if interval.end_excl() > 20 {
                    return None;
                }
                let sw = crate::StackWindow::new(&stack, interval);
                let runs: Vec<_> = sw.iter_height_runs().collect();
                Some(((sw.interval().start(), sw.interval().end_excl()), runs))
            })
            .collect();

        assert_eq!(
            cursor_windows, oracle_windows,
            "mismatch for intervals: {intervals:?}"
        );
    }
}

// ---------------------------------------------------------------------------
// WindowIter: DoubleEndedIterator
// ---------------------------------------------------------------------------

#[test]
fn window_iter_forward_matches_oracle() {
    let stack = stack_from_intervals(&[(1, 4), (3, 6), (8, 10)]);

    let cursor_iter: Vec<_> = stack
        .iter_windows(0, 11, 5u32)
        .map(|w| (w.interval().start(), w.interval().end_excl()))
        .collect();

    let oracle: Vec<_> = (0..7)
        .filter_map(|i| {
            let interval = I32CO::checked_from_start_len(i, 5u32)?;
            (interval.end_excl() <= 11).then_some((interval.start(), interval.end_excl()))
        })
        .collect();

    assert_eq!(cursor_iter, oracle);
}

#[test]
fn window_iter_backward_matches_oracle() {
    let stack = stack_from_intervals(&[(1, 4), (3, 6), (8, 10)]);

    let cursor_iter: Vec<_> = stack
        .iter_windows(0, 11, 5u32)
        .rev()
        .map(|w| (w.interval().start(), w.interval().end_excl()))
        .collect();

    let oracle: Vec<_> = (0..7)
        .rev()
        .filter_map(|i| {
            let interval = I32CO::checked_from_start_len(i, 5u32)?;
            (interval.end_excl() <= 11).then_some((interval.start(), interval.end_excl()))
        })
        .collect();

    assert_eq!(cursor_iter, oracle);
}

#[test]
fn window_iter_mixed_next_and_next_back() {
    let stack = stack_from_intervals(&[(1, 4), (3, 6)]);

    // 5 windows: [0,3), [1,4), [2,5), [3,6), [4,7)
    let mut iter = stack.iter_windows(0, 7, 3u32);

    // Front: [0,3)
    let w = iter.next().unwrap();
    assert_eq!((w.interval().start(), w.interval().end_excl()), (0, 3));

    // Back: [4,7)
    let w = iter.next_back().unwrap();
    assert_eq!((w.interval().start(), w.interval().end_excl()), (4, 7));

    // Front: [1,4)
    let w = iter.next().unwrap();
    assert_eq!((w.interval().start(), w.interval().end_excl()), (1, 4));

    // Back: [3,6)
    let w = iter.next_back().unwrap();
    assert_eq!((w.interval().start(), w.interval().end_excl()), (3, 6));

    // Front: [2,5) (last one)
    let w = iter.next().unwrap();
    assert_eq!((w.interval().start(), w.interval().end_excl()), (2, 5));

    assert_eq!(iter.len(), 0);
    assert!(iter.next().is_none());
    assert!(iter.next_back().is_none());
}

#[test]
fn window_iter_empty_returns_none_immediately() {
    let stack: IntCOStack<I32CO> = IntCOStack::default();

    // Invalid domain from > to
    let mut iter = stack.iter_windows(5, 3, 1u32);
    assert_eq!(iter.len(), 0);
    assert!(iter.next().is_none());
    assert!(iter.next_back().is_none());

    // Zero len
    let mut iter = stack.iter_windows(0, 5, 0u32);
    assert_eq!(iter.len(), 0);
    assert!(iter.next().is_none());
    assert!(iter.next_back().is_none());

    // Len > domain
    let mut iter = stack.iter_windows(0, 3, 5u32);
    assert_eq!(iter.len(), 0);
    assert!(iter.next().is_none());
    assert!(iter.next_back().is_none());
}

#[test]
fn window_iter_exact_size() {
    let stack = stack_from_intervals(&[(1, 4), (3, 6)]);

    let iter = stack.iter_windows(0, 10, 3u32);
    assert_eq!(iter.len(), 8);

    let collected: Vec<_> = iter.collect();
    assert_eq!(collected.len(), 8);
}

#[test]
fn window_iter_all_windows_covered_once() {
    let stack = stack_from_intervals(&[(0, 2), (4, 6), (8, 10)]);

    let mut seen = std::collections::HashSet::new();
    let mut iter = stack.iter_windows(-2, 12, 3u32);

    while let Some(w) = iter.next() {
        let key = (w.interval().start(), w.interval().end_excl());
        assert!(seen.insert(key), "duplicate window {key:?}");
    }

    // With from=-2, to=12, len=3: 12 windows
    assert_eq!(seen.len(), 12);
}

// ---------------------------------------------------------------------------
// WindowIter with I8CO (signed, small domain)
// ---------------------------------------------------------------------------

#[test]
fn window_iter_i8co_small_domain() {
    let intervals: Vec<I8CO> = [(0i8, 5i8), (3i8, 7i8)]
        .iter()
        .copied()
        .map(|(s, e)| I8CO::try_new(s, e).unwrap())
        .collect();
    let stack: IntCOStack<I8CO> = intervals.into_iter().collect();

    let cursor_iter: Vec<_> = stack
        .iter_windows(0i8, 7i8, 3u8)
        .map(|w| (w.interval().start(), w.interval().end_excl()))
        .collect();

    // windows: [0,3), [1,4), [2,5), [3,6), [4,7)
    let expected = vec![(0, 3), (1, 4), (2, 5), (3, 6), (4, 7)];
    assert_eq!(cursor_iter, expected);
}

// ---------------------------------------------------------------------------
// Height-run integrity
// ---------------------------------------------------------------------------

#[test]
fn cursor_height_runs_match_stack_window_height_runs() {
    let stack = stack_from_intervals(&[(1, 4), (2, 7), (5, 10)]);

    for window in stack.iter_windows(0, 12, 4u32) {
        let cursor_runs: Vec<_> = window.iter_height_runs().collect();

        // Reconstruct via StackWindow::new (binary search) as oracle
        let interval = I32CO::checked_from_start_len(window.interval().start(), 4u32).unwrap();
        let sw = crate::StackWindow::new(&stack, interval);
        let oracle_runs: Vec<_> = sw.iter_height_runs().collect();

        assert_eq!(
            cursor_runs,
            oracle_runs,
            "run mismatch at window [{}, {})",
            window.interval().start(),
            window.interval().end_excl()
        );
    }
}

// ---------------------------------------------------------------------------
// Serial / parallel equivalence
// ---------------------------------------------------------------------------

#[test]
fn cursor_serial_and_parallel_windows_match() {
    use rayon::prelude::*;

    let stack = stack_from_intervals(&[(1, 4), (3, 6), (8, 10)]);

    let serial: Vec<_> = stack
        .iter_windows(0, 11, 5u32)
        .map(|w| {
            let runs: Vec<_> = w.iter_height_runs().collect();
            ((w.interval().start(), w.interval().end_excl()), runs)
        })
        .collect();

    let parallel: Vec<_> = stack
        .par_iter_windows(0, 11, 5u32)
        .map(|w| {
            let runs: Vec<_> = w.iter_height_runs().collect();
            ((w.interval().start(), w.interval().end_excl()), runs)
        })
        .collect();

    assert_eq!(serial, parallel);
}
