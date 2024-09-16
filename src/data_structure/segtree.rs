use cargo_snippet::snippet;

use crate::algebra::Monoid;
// use crate::{algebra::Monoid, utils::range_to_tuple};


#[snippet("Segtree")]
#[derive(Debug)]
pub struct Segtree<T> {
    original_size: usize,
    leaf_size: usize,
    node: Vec<T>,
}
#[snippet("Segtree")]
impl<T> Segtree<T>
where
    T: Monoid + Clone,
{
    pub fn new(n: usize) -> Self {
        let exp = {
            let mut ok: i64 = 41;
            let mut ng: i64 = -1;
            while (ok - ng).abs() > 1 {
                let mid = (ok + ng) / 2;
                if 1 << mid >= n {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok
        };
        let size = 1 << exp;
        Segtree {
            original_size: n,
            leaf_size: size,
            node: vec![T::e(); 2 * size],
        }
    }
    pub fn build(&mut self, vec: &[T]) {
        for (i, ele) in vec.iter().enumerate() {
            let idx = i + self.leaf_size;
            self.node[idx] = ele.clone();
        }
        for i in (1..self.leaf_size).rev() {
            self.node[i] = self.node[i << 1].op(&self.node[i << 1 | 1])
        }
    }
    pub fn update(&mut self, i: usize, x: T) {
        //! 一点更新 O(logN)
        let mut idx = i + self.leaf_size;
        self.node[idx] = x;
        while idx > 1 {
            idx >>= 1;
            self.node[idx] = self.node[idx << 1].op(&self.node[idx << 1 | 1])
        }
    }

    pub fn add(&mut self, idx: usize, x: T) {
        //! 一点に`x`との二項演算 O(log N)
        let mut idx = idx + self.leaf_size;
        self.node[idx] = self.node[idx].op(&x);
        while idx > 1 {
            idx >>= 1;
            self.node[idx] = self.node[idx << 1].op(&self.node[idx << 1 | 1])
        }
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
    
    pub fn fold<R>(&self, range: R) -> T
    where
        R: std::ops::RangeBounds<usize>,
    {
        //! 区間取得 O(logN)
        let (l, r) = Self::range_to_tuple(range, self.original_size);
        self.query(l, r)
    }
    fn query(&self, l: usize, r: usize) -> T {
        // if !(l < self.original_size) || !(1 <= r && r <= self.original_size) {
        //     return T::e();
        // }
        assert!(l < self.original_size);
        assert!(1 <= r && r <= self.original_size);

        let mut l = l + self.leaf_size;
        let mut r = r + self.leaf_size;
        let mut vl = T::e();
        let mut vr = T::e();

        while l < r {
            if l & 1 == 1 {
                vl = vl.op(&self.node[l].clone());
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                vr = self.node[r].op(&vr);
            }
            l >>= 1;
            r >>= 1;
        }
        vl.op(&vr)
    }
    pub fn get(&self, idx: usize) -> &T {
        &self.node[idx + self.leaf_size]
    }
    pub fn max_right<F>(&self, mut l: usize, f: F) -> usize
    where
        F: Fn(&T) -> bool,
    {
        //! `f(l..r) == true`となる最大のrを探索
        assert!(l <= self.original_size);
        assert!(f(&T::e()));

        l += self.leaf_size;
        let mut v = T::e();

        loop {
            // 右ノードになるまで親に移動
            while l & 1 != 1 {
                l >>= 1;
            }
            // ここまでの間で、始めてfalseになるrがあるので、そこを探索
            // lの部分木の葉に着くのが目的
            if !f(&v.op(&self.node[l])) {
                while l < self.leaf_size {
                    // 左ノードに移動
                    l <<= 1;
                    // 左ノードがtrueなら右ノードがfalse
                    if f(&v.op(&self.node[l])) {
                        v = v.op(&self.node[l]);
                        l += 1;
                    }
                }
                return l - self.leaf_size;
            }
            // ここまで、trueなので、総積に加える
            v = v.op(&self.node[l]);
            l += 1; // 隣のノードに移動し、また区間を二倍にしつつ探索するloopに入る
            {
                let l = l as isize;
                if l & -l == l {
                    break;
                }
            }
        }
        self.original_size
    }

    pub fn min_left<F>(&self, mut r: usize, f: F) -> usize
    where
        F: Fn(&T) -> bool,
    {
        //! `f(l..r) == true`となる最小のlを探索

        assert!(r <= self.original_size);
        assert!(f(&T::e()));

        r += self.leaf_size;
        let mut v = T::e();

        loop {
            r -= 1;
            // 左ノードになるまで
            while r > 1 && r & 1 == 1 {
                r >>= 1;
            }
            if !f(&self.node[r].op(&v)) {
                while r < self.leaf_size {
                    r <<= 1;
                    r += 1;
                    if f(&self.node[r].op(&v)) {
                        v = self.node[r].op(&v);
                        r -= 1;
                    }
                }
                return r + 1 - self.leaf_size;
            }
            v = self.node[r].op(&v);
            {
                let r = r as isize;
                if r & -r == r {
                    break;
                }
            }
        }
        0
    }
}
