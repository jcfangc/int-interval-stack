use int_interval::I32CO;
use int_interval_stack::IntCOStack;

pub(crate) type Bounds = (i32, i32);

#[allow(dead_code)]
#[inline]
pub(crate) fn iv(start: i32, end_excl: i32) -> I32CO {
    I32CO::try_new(start, end_excl).unwrap()
}

#[allow(dead_code)]
#[inline]
pub(crate) fn stack_from_bounds(bounds: &[Bounds]) -> IntCOStack<I32CO> {
    bounds.iter().copied().map(|(s, e)| iv(s, e)).collect()
}

pub(crate) fn sorted_disjoint(n: usize) -> Vec<Bounds> {
    (0..n)
        .map(|i| {
            let start = i as i32 * 4;
            (start, start + 2)
        })
        .collect()
}

pub(crate) fn reversed_disjoint(n: usize) -> Vec<Bounds> {
    let mut v = sorted_disjoint(n);
    v.reverse();
    v
}

pub(crate) fn adjacent_chain(n: usize) -> Vec<Bounds> {
    (0..n)
        .map(|i| {
            let start = i as i32 * 2;
            (start, start + 2)
        })
        .collect()
}

pub(crate) fn nested_dense(n: usize) -> Vec<Bounds> {
    (0..n)
        .map(|i| {
            let start = i as i32;
            let end_excl = (n * 2) as i32 - i as i32;
            (start, end_excl.max(start + 1))
        })
        .collect()
}

pub(crate) fn shifted_overlap(n: usize) -> Vec<Bounds> {
    (0..n)
        .map(|i| {
            let start = i as i32;
            (start, start + 32)
        })
        .collect()
}

pub(crate) fn mixed_unsorted(n: usize) -> Vec<Bounds> {
    let groups = n.div_ceil(4);
    let mut v = Vec::with_capacity(groups * 4);

    for i in 0..groups {
        let base = i as i32 * 40;
        v.extend([
            (base + 8, base + 18),
            (base, base + 10),
            (base + 24, base + 30),
            (base + 18, base + 24),
        ]);
    }

    v.reverse();
    v.truncate(n);
    v
}

pub(crate) fn cases(n: usize) -> Vec<(&'static str, Vec<Bounds>)> {
    vec![
        ("sorted_disjoint", sorted_disjoint(n)),
        ("reversed_disjoint", reversed_disjoint(n)),
        ("adjacent_chain", adjacent_chain(n)),
        ("nested_dense", nested_dense(n)),
        ("shifted_overlap", shifted_overlap(n)),
        ("mixed_unsorted", mixed_unsorted(n)),
    ]
}

#[allow(dead_code)]
pub(crate) fn point_queries(bounds: &[Bounds]) -> Vec<i32> {
    let mut out = Vec::with_capacity(bounds.len().min(128) * 3 + 2);

    for &(s, e) in bounds.iter().take(128) {
        out.push(s);
        out.push(s + (e - s) / 2);
        out.push(e);
    }

    if let Some(&(s, _)) = bounds.first() {
        out.push(s - 1);
    }
    if let Some(&(_, e)) = bounds.last() {
        out.push(e + 1);
    }

    out.sort_unstable();
    out.dedup();
    out
}
