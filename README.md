# int-interval-stack

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

```text
ChangePoint { at, height_after }
```

Each change point means that immediately after `at`, the active stack height becomes `height_after`.

The representation is canonical:

- change-point coordinates are strictly increasing;
- adjacent change points always have different heights;
- redundant zero-net boundaries are omitted;
- the final change point restores the height to zero when the stack is non-empty.

## Installation

```toml
[dependencies]
int-interval-stack = "0.1"
int-interval = "0.9"
```

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

assert_eq!(stack.max_height(), 3);
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

## Iterating covered segments

`iter_intervals()` returns canonical covered segments and their heights.

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
    .iter_intervals()
    .map(|(iv, height)| ((iv.start(), iv.end_excl()), height))
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

## Height-filtered iteration

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
    .peak_intervals()
    .map(|(iv, height)| ((iv.start(), iv.end_excl()), height))
    .collect();

assert_eq!(peak, vec![((5, 7), 3)]);
```

Available iterators:

```rust
stack.iter_intervals();
stack.iter_intervals_at_least(2);
stack.iter_intervals_at_most(2);
stack.iter_intervals_exactly(2);
stack.iter_intervals_between(1, 3);
stack.peak_intervals();
```

## Predicates

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

assert!(stack.contains_point(5));
assert!(!stack.contains_point(15));

assert!(stack.intersects_interval(iv(8, 12)));
assert!(!stack.intersects_interval(iv(12, 18)));

assert!(stack.contains_interval(iv(2, 8)));
assert!(!stack.contains_interval(iv(8, 22)));
```

## Parallel construction

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

assert_eq!(stack.max_height(), 3);
```

Parallel construction is intended for larger inputs. For small collections, sequential construction may be faster due to Rayon scheduling overhead.

## Complexity

Let `n` be the number of input intervals and `m` be the number of canonical change points.

| Operation                    |                                  Complexity |
| ---------------------------- | ------------------------------------------: |
| Build from iterator          |  `O(n log n)` dominated by endpoint sorting |
| Build from parallel iterator | parallel endpoint compaction plus reduction |
| `height_at(x)`               |                                  `O(log m)` |
| `contains_point(x)`          |                                  `O(log m)` |
| `max_height()`               |                                      `O(m)` |
| `iter_intervals()`           |                                      `O(m)` |
| `intersects_interval(query)` |                              `O(log m + k)` |
| `contains_interval(query)`   |                              `O(log m + k)` |

Here `k` is the number of change points scanned inside the query range.

## Benchmarks

This crate includes Criterion / CodSpeed benchmarks for construction, point queries, interval predicates, height-filtered iteration, peak iteration, and parallel construction.

Run locally:

```bash
cargo bench
```

Generate the longer report profile:

```bash
BENCH_PROFILE=report cargo bench
```

## License

Licensed under either of:

- MIT license
- Apache License, Version 2.0
