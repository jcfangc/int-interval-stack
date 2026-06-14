# int-interval-stack

[![Crates.io](https://img.shields.io/crates/v/int-interval-stack.svg)](https://crates.io/crates/int-interval-stack)
[![Documentation](https://docs.rs/int-interval-stack/badge.svg)](https://docs.rs/int-interval-stack)
[![License](https://img.shields.io/crates/l/int-interval-stack.svg)](https://crates.io/crates/int-interval-stack)
[![CodSpeed](https://github.com/jcfangc/int-interval-stack/actions/workflows/codspeed.yml/badge.svg?branch=main)](https://github.com/jcfangc/int-interval-stack/actions/workflows/codspeed.yml)
[![Coverage](https://codecov.io/gh/jcfangc/int-interval-stack/branch/main/graph/badge.svg)](https://codecov.io/gh/jcfangc/int-interval-stack)


Canonical stack-height functions for integer half-open intervals.

`int-interval-stack` builds an immutable representation of interval overlap multiplicity. Given a collection of half-open integer intervals, it stores the resulting piecewise-constant height function as canonical change points.

```text
[start, end) contributes +1 height on every covered coordinate.
```

This is useful when a boolean interval set is not enough and the number of overlapping intervals matters.

## Model

Input intervals are interpreted as half-open ranges:

```text
[start, end)
```

The stack height at coordinate `x` is:

```text
height_at(x) = number of input intervals containing x
```

Internally, the stack is represented as ordered change points:

```rust
ChangePoint { at, height_after }
```

Each change point means that from `at` onward, until the next change point, the active stack height is `height_after`.

The representation is canonical:

* change-point coordinates are strictly increasing;
* adjacent change points always have different heights;
* redundant zero-net boundaries are omitted;
* the final change point restores the height to zero when the stack is non-empty.

## Installation

```toml
[dependencies]
int-interval-stack = "0.3"
int-interval = "0.9"
```

Enable `rayon` in your own crate when using parallel collection or parallel iteration.

## Basic usage

```rust
use int_interval::I32CO;
use int_interval_stack::IntCOStack;

fn iv(start: i32, end_excl: i32) -> I32CO {
    I32CO::try_new(start, end_excl).unwrap()
}

let stack: IntCOStack<I32CO> = [
    iv(0, 10),
    iv(3, 7),
    iv(5, 12),
]
.into_iter()
.collect();

assert_eq!(stack.height_at(2), 1);
assert_eq!(stack.height_at(4), 2);
assert_eq!(stack.height_at(6), 3);
assert_eq!(stack.height_at(11), 1);
assert_eq!(stack.height_at(12), 0);

assert_eq!(stack.height_stats().max_height(), 3);
```

## Change points

```rust
use int_interval::I32CO;
use int_interval_stack::IntCOStack;

fn iv(start: i32, end_excl: i32) -> I32CO {
    I32CO::try_new(start, end_excl).unwrap()
}

let stack: IntCOStack<I32CO> = [
    iv(0, 10),
    iv(3, 7),
]
.into_iter()
.collect();

let points = stack.change_points();

assert_eq!(points[0].at, 0);
assert_eq!(points[0].height_after, 1);

assert_eq!(points[1].at, 3);
assert_eq!(points[1].height_after, 2);

assert_eq!(points[2].at, 7);
assert_eq!(points[2].height_after, 1);

assert_eq!(points[3].at, 10);
assert_eq!(points[3].height_after, 0);
```

## Covered set

`covered()` projects the stack to a canonical interval set containing all coordinates whose stack height is positive.

```rust
use int_interval::traits::IntCO;
use int_interval::I32CO;
use int_interval_stack::IntCOStack;

fn iv(start: i32, end_excl: i32) -> I32CO {
    I32CO::try_new(start, end_excl).unwrap()
}

let stack: IntCOStack<I32CO> = [
    iv(0, 10),
    iv(3, 7),
    iv(20, 30),
]
.into_iter()
.collect();

let covered: Vec<_> = stack
    .covered()
    .iter_intervals()
    .map(|iv| (iv.start(), iv.end_excl()))
    .collect();

assert_eq!(covered, vec![(0, 10), (20, 30)]);
```

The covered set is built lazily and cached after the first call.

## Height segments

`iter_height_segments()` iterates over positive-height segments. Each segment is
a closed-open interval together with the positive stack height that applies
throughout that interval.

```rust
use int_interval::traits::IntCO;
use int_interval::I32CO;
use int_interval_stack::IntCOStack;

fn iv(start: i32, end_excl: i32) -> I32CO {
    I32CO::try_new(start, end_excl).unwrap()
}

let stack: IntCOStack<I32CO> = [
    iv(0, 10),
    iv(3, 7),
]
.into_iter()
.collect();

let segments: Vec<_> = stack
    .iter_height_segments()
    .map(|seg| {
        (
            (seg.interval.start(), seg.interval.end_excl()),
            seg.height.get(),
        )
    })
    .collect();

assert_eq!(
    segments,
    vec![
        ((0, 3), 1),
        ((3, 7), 2),
        ((7, 10), 1),
    ]
);
```

Zero-height gaps are skipped by the positive-height segment iterators.

## Height-filtered segments

The stack can iterate only segments whose height matches a selected condition.

```rust
use int_interval::traits::IntCO;
use int_interval::I32CO;
use int_interval_stack::IntCOStack;

fn iv(start: i32, end_excl: i32) -> I32CO {
    I32CO::try_new(start, end_excl).unwrap()
}

let stack: IntCOStack<I32CO> = [
    iv(0, 10),
    iv(3, 7),
    iv(5, 12),
]
.into_iter()
.collect();

let peak: Vec<_> = stack
    .iter_peak_height_segments()
    .map(|seg| {
        (
            (seg.interval.start(), seg.interval.end_excl()),
            seg.height.get(),
        )
    })
    .collect();

assert_eq!(peak, vec![((5, 7), 3)]);
```

Available positive-height segment iterators:

```rust
stack.iter_height_segments();
stack.iter_height_segments_at_least(2);
stack.iter_height_segments_at_most(2);
stack.iter_height_segments_exactly(2);
stack.iter_height_segments_between(1, 3);
stack.iter_peak_height_segments();
```

These iterators use height statistics for cheap fast paths. For example, queries above the observed maximum height can return an empty iterator immediately, and uniform-height stacks can reuse the covered-set shape.

## Window iteration

`iter_windows(from, to, len)` iterates over all fixed-length coordinate windows fully contained in `[from, to)`.

Window starts advance by one coordinate unit:

```text
[from,     from + len)
[from + 1, from + 1 + len)
[from + 2, from + 2 + len)
...
```

Each yielded `StackWindow` can be decomposed into constant-height runs. Unlike `iter_height_segments()`, window height runs include zero-height gaps inside the window.

```rust
use int_interval::traits::IntCO;
use int_interval::I32CO;
use int_interval_stack::IntCOStack;

fn iv(start: i32, end_excl: i32) -> I32CO {
    I32CO::try_new(start, end_excl).unwrap()
}

let stack: IntCOStack<I32CO> = [
    iv(2, 5),
    iv(4, 8),
]
.into_iter()
.collect();

let windows: Vec<_> = stack
    .iter_windows(0, 10, 4)
    .map(|window| {
        let bounds = (
            window.interval().start(),
            window.interval().end_excl(),
        );

        let runs: Vec<_> = window
            .iter_height_runs()
            .map(|run| {
                (
                    (run.interval.start(), run.interval.end_excl()),
                    run.height,
                )
            })
            .collect();

        (bounds, runs)
    })
    .collect();

assert_eq!(windows[0].0, (0, 4));
assert_eq!(
    windows[0].1,
    vec![
        ((0, 2), 0),
        ((2, 4), 1),
    ]
);
```

Invalid window geometries produce an empty iterator:

* `from >= to`;
* `len == 0`;
* `len` is greater than the length of `[from, to)`;
* the window count cannot be represented as `usize`.

## Parallel iteration

`IntCOStack` implements Rayon parallel collection.

```rust
use int_interval::I32CO;
use int_interval_stack::IntCOStack;
use rayon::prelude::*;

fn iv(start: i32, end_excl: i32) -> I32CO {
    I32CO::try_new(start, end_excl).unwrap()
}

let intervals = vec![
    iv(0, 10),
    iv(3, 7),
    iv(5, 12),
];

let stack: IntCOStack<I32CO> = intervals
    .into_par_iter()
    .collect();

assert_eq!(stack.height_stats().max_height(), 3);
```

Window iteration also has a parallel form:

```rust
use int_interval::traits::IntCO;
use int_interval::I32CO;
use int_interval_stack::IntCOStack;
use rayon::prelude::*;

fn iv(start: i32, end_excl: i32) -> I32CO {
    I32CO::try_new(start, end_excl).unwrap()
}

let stack: IntCOStack<I32CO> = [
    iv(2, 5),
    iv(4, 8),
]
.into_iter()
.collect();

let bounds: Vec<_> = stack
    .par_iter_windows(0, 10, 4)
    .map(|window| {
        (
            window.interval().start(),
            window.interval().end_excl(),
        )
    })
    .collect();

assert_eq!(bounds[0], (0, 4));
```

`StackWindow` also supports parallel height-run iteration:

```rust
use int_interval::I32CO;
use int_interval_stack::IntCOStack;
use rayon::prelude::*;

fn iv(start: i32, end_excl: i32) -> I32CO {
    I32CO::try_new(start, end_excl).unwrap()
}

let stack: IntCOStack<I32CO> = [
    iv(2, 5),
    iv(4, 8),
]
.into_iter()
.collect();

let window = stack.iter_windows(0, 10, 4).next().unwrap();

let run_count = window
    .par_iter_height_runs()
    .count();

assert_eq!(run_count, 2);
```

Parallel construction and parallel iteration are intended for larger workloads. For small inputs or short windows, sequential iteration may be faster due to Rayon scheduling overhead.

## Covered-set queries

`IntCOStack` stores overlap multiplicity. Boolean coverage queries are delegated to the projected covered set.

Use `covered()` to get the canonical interval set of all coordinates whose stack height is positive, then call the `int-interval-set` query methods on that set.

```rust
use int_interval::I32CO;
use int_interval_stack::IntCOStack;

fn iv(start: i32, end_excl: i32) -> I32CO {
    I32CO::try_new(start, end_excl).unwrap()
}

let stack: IntCOStack<I32CO> = [
    iv(0, 10),
    iv(20, 30),
]
.into_iter()
.collect();

let covered = stack.covered();

assert!(covered.contains_point(5));
assert!(!covered.contains_point(15));

assert!(covered.intersects_interval(iv(8, 12)));
assert!(!covered.intersects_interval(iv(12, 18)));

assert!(covered.contains_interval(iv(2, 8)));
assert!(!covered.contains_interval(iv(8, 22)));
```

The covered set is built lazily from the stack change points and cached after the first call. This keeps stack construction focused on the height function while reusing `int-interval-set` for boolean interval-set queries.

## Complexity

Let:

* `n` be the number of input intervals;
* `m` be the number of canonical stack change points;
* `c` be the number of intervals in the projected covered set;
* `w` be the number of emitted coordinate windows;
* `q` be the number of stack change points strictly inside a single window;
* `r` be the number of height runs emitted by a window workload.

| Operation                              |                                                   Complexity |
| -------------------------------------- | -----------------------------------------------------------: |
| Build from iterator                    |                   `O(n log n)` dominated by endpoint sorting |
| Build from parallel iterator           |                  parallel endpoint compaction plus reduction |
| `change_points()`                      |                                                       `O(1)` |
| `height_stats()`                       |                                                       `O(1)` |
| `height_at(x)`                         |                                                   `O(log m)` |
| `covered()` first call                 |                       `O(m)` to project positive-height runs |
| `covered()` after cache initialization |                    `O(1)` to return the cached set reference |
| `covered().contains_point(x)`          |                              delegated to `int-interval-set` |
| `covered().intersects_interval(query)` |                              delegated to `int-interval-set` |
| `covered().contains_interval(query)`   |                              delegated to `int-interval-set` |
| `iter_height_segments()`               |                                            `O(m)` worst case |
| height-filtered segment iteration      |           `O(m)` worst case, with statistic-based fast paths |
| `iter_windows(from, to, len)`          |    `O(w)` window objects, excluding per-window run iteration |
| `StackWindow::new(stack, interval)`    |     `O(log m)` to locate the window-local change-point range |
| `window.height_run_count()`            |                                                       `O(1)` |
| `window.iter_height_runs()`            |                                      `O(q + 1)` emitted runs |
| `par_iter_windows(from, to, len)`      |                  indexed parallel iteration over `w` windows |
| `window.par_iter_height_runs()`        | indexed parallel iteration over the window-local height runs |

The stack itself does not duplicate covered-set predicate APIs. Exact complexity for boolean set queries is defined by `int-interval-set`.

Window height runs include zero-height gaps. Therefore a window with no interior change points still emits one run covering the full window.

## Dense-window baseline

A dense alternative is to materialize one height value per coordinate into a `Vec<usize>` and use `std::slice::windows(len)` plus local run compression.

This baseline is strong when:

* the coordinate domain is small;
* the dense height vector already exists;
* windows are short;
* the height function is highly fragmented.

`IntCOStack` window iteration is designed for cases where the stack already exists as canonical change points, the coordinate domain may be large or sparse, or each window can be represented by relatively few height runs.

In short:

```text
dense Vec + slice::windows:
    strong for small dense domains and short windows

IntCOStack::iter_windows:
    useful when the stack has a compact run representation or windows are long
```

## Benchmarks

This crate includes Criterion / CodSpeed benchmarks for:

* construction;
* parallel construction;
* point height queries;
* positive-height segment iteration;
* height-filtered segment iteration;
* peak-height segment iteration;
* window iteration;
* parallel window iteration;
* window height-run iteration;
* dense-window baseline comparison.

Run locally:

```bash
cargo bench
```

Run a single benchmark:

```bash
cargo bench --bench window_dense_baseline
```

Generate the longer report profile:

```bash
BENCH_PROFILE=report cargo bench
```

The dense-window benchmark compares three modes:

```text
int_interval_stack:
    stack.iter_windows(...).flat_map(window.iter_height_runs())

std_dense_query_only:
    precomputed dense Vec<usize> + std::slice::windows(...)

std_dense_end_to_end:
    materialize dense heights from the stack, then use std::slice::windows(...)
```

The dense query-only baseline is intentionally strong. It does not include the cost of constructing the dense height vector.

## License

Licensed under either of:

* MIT license
* Apache License, Version 2.0
