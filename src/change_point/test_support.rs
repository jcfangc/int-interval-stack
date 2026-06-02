use super::*;

pub(crate) fn oracle_points(intervals: &[(i32, i32)]) -> Vec<ChangePoint<i32>> {
    let ats: BTreeSet<i32> = intervals.iter().flat_map(|&(s, e)| [s, e]).collect();
    let mut prev = 0usize;
    let mut out = Vec::new();

    for at in ats {
        let next = naive_height_at(intervals, at);
        if next != prev {
            out.push(ChangePoint {
                at,
                height_after: next,
            });
            prev = next;
        }
    }

    out
}
