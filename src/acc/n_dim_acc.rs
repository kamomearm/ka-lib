#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}

pub struct Acc3d {
    acc: Vec<Vec<Vec<isize>>>,
}
impl Acc3d {
    pub fn new(v: &[Vec<Vec<isize>>]) -> Self {
        let n = v.len();
        let m = v[0].len();
        let w = v[0][0].len();
        let mut acc = vec![vec![vec![0; w + 1]; m + 1]; n + 1];
        for i in 1..=n {
            for j in 1..=m {
                for k in 1..=w {
                    acc[i][j][k] = acc[i - 1][j][k] + v[i - 1][j - 1][k - 1]
                }
            }
        }
        for i in 1..=n {
            for j in 1..=m {
                for k in 1..=w {
                    acc[i][j][k] += acc[i][j - 1][k];
                }
            }
        }
        for i in 1..=n {
            for j in 1..=m {
                for k in 1..=w {
                    acc[i][j][k] += acc[i][j][k - 1];
                }
            }
        }
        Acc3d { acc }
    }
    fn range_to_tuple<R>(range: R, r_max: usize) -> (usize, usize)
    where
        R: std::ops::RangeBounds<usize>,
    {
        use std::ops::Bound;
        let l = match range.start_bound() {
            Bound::Included(l) => *l,
            Bound::Excluded(l) => l + 1,
            Bound::Unbounded => 0,
        };
        let r = match range.end_bound() {
            Bound::Included(r) => r + 1,
            Bound::Excluded(r) => *r,
            Bound::Unbounded => r_max,
        };
        (l, r)
    }
    pub fn fold<R>(&self, range_i: R, range_j: R, range_k: R) -> isize
    where
        R: std::ops::RangeBounds<usize>,
    {
        let (li, ri) = Self::range_to_tuple(range_i, self.acc.len());
        let (lj, rj) = Self::range_to_tuple(range_j, self.acc[0].len());
        let (lk, rk) = Self::range_to_tuple(range_k, self.acc[0][0].len());
        // self.acc[ri][rj][rk] + self.acc[ri][lj][lk] + self.acc[li][rj][lk] + self.acc[li][lj][rk]
        //     - self.acc[li][rj][rk]
        //     - self.acc[ri][lj][rk]
        //     - self.acc[ri][rj][lk]
        //     - self.acc[li][lj][lk]
        let sum_k = |ri: usize, rj: usize, lk: usize, rk: usize| {
            self.acc[ri][rj][rk] - self.acc[ri][rj][lk]
        };
        let sum_jk = |ri: usize, lj: usize, rj: usize, lk: usize, rk: usize| {
            sum_k(ri, rj, lk, rk) - sum_k(ri, lj, lk, rk)
        };
        let sum_ijk = |li: usize, ri: usize, lj: usize, rj: usize, lk: usize, rk: usize| {
            sum_jk(ri, lj, rj, lk, rk) - sum_jk(li, lj, rj, lk, rk)
        };
        sum_ijk(li, ri, lj, rj, lk, rk)
    }
}
pub struct Acc2d {
    acc: Vec<Vec<isize>>,
}
impl Acc2d {
    pub fn new(v: &[Vec<isize>]) -> Self {
        let n = v.len();
        let m = v[0].len();
        let mut acc = vec![vec![0; m + 1]; n + 1];
        for i in 1..=n {
            for j in 1..=m {
                acc[i][j] = acc[i - 1][j] + v[i - 1][j - 1];
            }
        }
        for j in 1..=m {
            for i in 1..=n {
                acc[i][j] += acc[i][j - 1];
            }
        }
        Acc2d { acc }
    }
    pub fn fold<R>(&self, range_i: R, range_j: R) -> isize
    where
        R: std::ops::RangeBounds<usize>,
    {
        // rangeを半開区間に直す
        // 0..2 -> l = 0, r = 2,
        // 0..=2 -> l = 0, r = 3,
        // 0.. -> l = 0, r = もとの配列の長さ+1
        use std::ops::Bound;
        let li = match range_i.start_bound() {
            Bound::Included(l) => *l,
            Bound::Excluded(l) => l + 1,
            Bound::Unbounded => 0,
        };
        let ri = match range_i.end_bound() {
            Bound::Included(r) => r + 1,
            Bound::Excluded(r) => *r,
            Bound::Unbounded => self.acc.len(),
        };
        let lj = match range_j.start_bound() {
            Bound::Included(l) => *l,
            Bound::Excluded(l) => l + 1,
            Bound::Unbounded => 0,
        };
        let rj = match range_j.end_bound() {
            Bound::Included(r) => r + 1,
            Bound::Excluded(r) => *r,
            Bound::Unbounded => self.acc.len(),
        };
        self.acc[ri][rj] + self.acc[li][lj] - self.acc[ri][lj] - self.acc[li][rj]
    }
}
