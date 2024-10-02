use cargo_snippet::snippet;
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
// pub trait RangeToTuple {
//     fn range_to_tuple<R>(range: R, r_sup: usize) -> (usize, usize)
//     where
//         R: std::ops::RangeBounds<usize>,
//     {
//         use std::ops::Bound;
//         let l = match range.start_bound() {
//             Bound::Included(l) => *l,
//             Bound::Excluded(l) => l + 1,
//             Bound::Unbounded => 0,
//         };
//         let r = match range.end_bound() {
//             Bound::Included(r) => r + 1,
//             Bound::Excluded(r) => *r,
//             Bound::Unbounded => r_sup,
//         };
//         (l, r)
//     }
// }
pub struct TimeKeeper {
    st: std::time::Instant,
}
impl TimeKeeper {
    pub fn new() -> TimeKeeper {
        TimeKeeper {
            st : std::time::Instant::now()
        }
    }
    pub fn elapsed_time(&self) -> std::time::Duration {
        self.st.elapsed()
    }
}

pub fn is_palindrome<T: Ord>(s: &[T]) -> bool {
    for i in 0..s.len() / 2 {
        if s[i] != s[s.len() - i - 1] {
            return false;
        }
    }
    true
}

/// from sansen
///
/// <https://judge.yosupo.jp/submission/15446>
// ---------- begin Scanner(require delimiter) ----------
pub mod scanner {
    pub struct Scanner<R> {
        reader: R,
        buf: Vec<u8>,
    }
    #[allow(dead_code)]
    impl<R: std::io::BufRead> Scanner<R> {
        pub fn new(reader: R) -> Self {
            Scanner {
                reader: reader,
                buf: Vec::with_capacity(1024),
            }
        }
        fn read(&mut self, del: u8) {
            self.buf.clear();
            self.reader.read_until(del, &mut self.buf).ok();
            assert!(self.buf.pop().unwrap() == del);
        }
        pub fn next<T: std::str::FromStr>(&mut self, del: u8) -> T {
            self.read(del);
            std::str::from_utf8(&self.buf)
                .unwrap()
                .trim()
                .parse::<T>()
                .ok()
                .unwrap()
        }
        pub fn next_bytes(&mut self, del: u8) -> Vec<u8> {
            self.read(del);
            std::str::from_utf8(&self.buf)
                .unwrap()
                .trim()
                .bytes()
                .collect()
        }
    }
}
// ---------- end scanner(require delimiter) ----------

#[snippet("CompMap")]
#[derive(Debug)]
pub struct CompMap<T> {
    original: Vec<T>,
}
#[snippet("CompMap")]
impl<T> CompMap<T>
where
    T: Clone + Ord,
{
    pub fn new() -> Self {
        CompMap { original: vec![] }
    }
    pub fn push(&mut self, x: T) {
        self.original.push(x);
    }
    pub fn push_vec(&mut self, x: &[T]) {
        for i in x {
            self.push(i.clone());
        }
    }
    // O(logN)
    pub fn get(&self, x: &T) -> usize {
        // self.original.lower_bound(x)
        self.lower_bound(x)
    }
    pub fn get_vec(&self, x: &[T]) -> Vec<usize> {
        let mut ret = vec![];
        for i in x {
            ret.push(self.get(i))
        }
        ret
    }
    // O(N logN)
    pub fn build(&mut self) {
        self.original.sort();
        self.original.dedup();
    }
    fn lower_bound(&self, x: &T) -> usize {
        let n = self.original.len();
        let mut ng = -1;
        let mut ok = n as isize;

        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if *x <= self.original[mid as usize] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    }
}
#[snippet("CompMap")]
impl<T> Default for CompMap<T>
where
    T: Clone + Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

pub fn is_good_bracket_str(s: &[char]) -> bool {
    let mut depth = 0;
    for &si in s {
        if si == '(' {
            depth += 1;
        } else {
            depth -= 1;
        }
        if depth < 0 {
            return false;
        }
    }
    if depth != 0 {
        return false;
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
