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
pub fn is_palindrome<T: Ord>(s: &[T]) -> bool {
    for i in 0..s.len() / 2 {
        if s[i] != s[s.len() - i - 1] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::utils::is_palindrome;



    #[test]
    fn it_works() {
        assert_eq!(is_palindrome(&[1, 2, 1]), true);
        assert_eq!(is_palindrome(&[1, 2, 3, 1]), false);
        assert_eq!(is_palindrome(&['a', 'b', 'a']), true);
    }
}
