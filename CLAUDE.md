# int-interval-stack

Canonical stack-height representation for integer half-open intervals — builds an immutable, piecewise-constant height function from interval overlap multiplicity.

## Architecture

```
                    ┌─────────────────────────────────┐
                    │         IntCOStack<I: IntCO>     │
                    │  ┌─────────────────────────────┐ │
                    │  │ Arc<[ChangePoint<C>]>        │ │  ← canonical, strictly-increasing, zero-final
                    │  │ HeightStats                  │ │  ← computed once at build time
                    │  │ OnceLock<IntCOSet<I>>        │ │  ← covered set, lazily cached
                    │  └─────────────────────────────┘ │
                    └────────┬───────────────┬────────┘
                             │               │
                    ┌────────▼────────┐  ┌───▼──────────────┐
                    │ HeightSegment<I>│  │ StackWindow<I>    │
                    │ interval: I     │  │ interval: I       │
                    │ height: NonZero │  │ start_idx: usize  │
                    └────────┬────────┘  └───┬───────────────┘
                             │               │
                    ┌────────▼────────┐  ┌───▼──────────────┐
                    │ HeightRun<I>    │  │ HeightRun<I>      │
                    │ height: usize   │  │ (zero-height      │
                    │ (always ≥ 1)    │  │  runs included)   │
                    └─────────────────┘  └──────────────────┘
```

### Core Types

- **`IntCOStack<I>`** — the central data structure. Generic over `I: IntCO` (closed-open integer interval). Holds an `Arc`'d slice of canonical change points, precomputed `HeightStats`, and a lazily-cached `IntCOSet` for the covered projection.

- **`ChangePoint<C>`** — a coordinate `at` plus `height_after`. Change points are strictly increasing in `at`, adjacent points always have different heights, and the final point restores `height_after = 0` when the stack is non-empty.

- **`HeightSegment<I>`** — a positive-height region: an interval paired with a `NonZeroUsize` height. Emitted by `iter_height_segments()` and its height-filtered variants.

- **`HeightRun<I>`** — a region of constant height (can be zero). Used by `StackWindow` iteration. Differs from `HeightSegment` in that it includes zero-height gaps.

- **`StackWindow<I>`** — a fixed-length coordinate window into the stack. Created cheaply (O(log m) to locate the window-local change-point range). Decomposable into `HeightRun`s.

- **`HeightStats`** — `min_positive_height_or_zero` and `max_height`. Enables fast-path short-circuiting in filtered segment iterators (e.g., above `max_height` → empty iterator; `is_uniform_positive_height()` → reuse covered-set shape).

### Type Aliases

`lib.rs` exports concrete aliases for all standard integer widths:

```rust
pub type I32COStack = IntCOStack<int_interval::I32CO>;
pub type U64COStack = IntCOStack<int_interval::U64CO>;
// … I8/I16/I32/I64/I128/Isize, U8/U16/U32/U64/U128/Usize
```

### Module Layout

```
src/
  lib.rs              — re-exports, type aliases
  int_co_stack.rs     — IntCOStack struct definition, submodule declarations
  int_co_stack/
    impls_for_access.rs         — height_at, change_points, height_stats, covered
    impls_for_construction.rs   — FromIterator, FromParallelIterator, merge_parts, build_parts
    impls_for_derived_traits.rs — Clone, Eq
    impls_for_iter.rs           — iter_height_segments and height-filtered variants
    impls_for_windows.rs        — iter_windows, window_at, start_at, window_count
    test_support.rs             — shared test fixtures for int_co_stack tests
  change_point.rs      — ChangePoint struct
  height_segment.rs    — HeightSegment struct, From<HeightSegment> for HeightRun
  height_stats.rs      — HeightStats struct
  height_run.rs        — HeightRun struct
  stack_window.rs      — StackWindow struct, height_run_at, iter_height_runs
```

## Key Invariants

### Canonical Change Points

1. Coordinates are strictly increasing.
2. Adjacent change points have different `height_after` values (no redundant points).
3. When the stack is non-empty, the final change point has `height_after = 0`.
4. Redundant zero-net boundaries are omitted.
5. These invariants are enforced during construction — never assumed on input.

### Height Stats

- Computed once during construction and never updated.
- `min_positive_height_or_zero` is 0 only when no positive height was observed.
- Used for fast-path decisions in filtered iterators — must be accurate.

### Covered Set Caching

- `covered()` returns a reference to a lazily-built `IntCOSet<I>`.
- Built on first call by projecting positive-height runs from change points.
- Subsequent calls return the cached value (O(1)).

## Inter-Crate Dependencies

| Crate | Version | Role |
|-------|---------|------|
| `int-interval` | 0.9.x | Closed-open integer interval trait (`IntCO`) and concrete types (`I32CO`, etc.) |
| `int-interval-set` | 0.3.x | Canonical boolean interval set — used for `covered()` projection |
| `either` | 1.x | Used in construction for endpoint compaction |
| `rayon` | 1.x | Parallel construction (`FromParallelIterator`) and parallel window/run iteration |

The sibling crates live at `../int-interval/` and `../int-interval-set/`.

## Benchmarks

Benchmarks use `divan` (via `codspeed-divan-compat`). All are under `benches/`:

```
benches/
  construct.rs              — sequential FromIterator construction
  construct_parallel.rs     — parallel FromParallelIterator construction
  height_at.rs              — point-height queries
  iter_height_segments.rs   — positive-height segment iteration (all filters)
  window_iter_bounds.rs     — sequential + parallel window iteration
  window_height_runs.rs     — window height-run decompression + dense baseline
```

Run all: `cargo bench`
Run one: `cargo bench --bench <name>`
Report profile (larger matrix): `BENCH_PROFILE=report cargo bench`

The dense-window baseline (`window_height_runs`) compares three modes: `IntCOStack::iter_windows` → `window.iter_height_runs()`, precomputed dense `Vec<usize>` + `slice::windows`, and end-to-end materialization from the stack.

## Testing

Tests follow the parent `CLAUDE.md` convention: `#[path = "..."]` subdirectory test modules with `tests_for_*` naming.

Shared test helpers live in `test_support.rs` modules co-located with the tests they serve (the `lab` convention from the parent CLAUDE.md — but this crate uses the name `test_support`).

Key test dependencies:
- `proptest` — property-based tests for construction invariants and round-trip properties.
- No doctests (`doctest = false` in `Cargo.toml`) — the README serves as documentation.

## Project-Specific Conventions

### No `unsafe`

This crate contains zero `unsafe` blocks. All operations are safe Rust.

### No `no_std`

Unlike some sibling crates, this crate uses `std` (for `Arc`, `OnceLock`, `Vec`). This is appropriate for the stack data structure.

### Parallelism is Opt-In

Rayon is a required dependency (not optional/feature-gated). Parallel construction and iteration are always available. For small inputs, sequential iteration may be faster — the caller chooses.

### Edition 2024

This crate uses Rust edition 2024.
