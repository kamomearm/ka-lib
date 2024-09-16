// use cargo_snippet::snippet;
// #[snippet]
// pub fn range_to_tuple<R>(range: R, r_max: usize) -> (usize, usize)
// where
//     R: std::ops::RangeBounds<usize>,
// {
//     use std::ops::Bound;
//     let l = match range.start_bound() {
//         Bound::Included(l) => *l,
//         Bound::Excluded(l) => l + 1,
//         Bound::Unbounded => 0,
//     };
//     let r = match range.end_bound() {
//         Bound::Included(r) => r + 1,
//         Bound::Excluded(r) => *r,
//         Bound::Unbounded => r_max,
//     };
//     (l, r)
// }
