use crate::{
    algebra::Monoid, utils::range_to_tuple
};
use cargo_snippet::snippet;

#[snippet("LazySegtree")]
pub trait ForLazySegtree {
    /// セグ木に乗せるモノイド
    type M: Monoid + Clone;
    /// Lazyに乗せるモノイド
    type L: Monoid + Clone;
    /// モノイドMに対して、Lをどう作用させるか
    fn mapping(m: &Self::M, l: &Self::L) -> Self::M;
}

#[snippet("LazySegtree")]
pub struct LazySegtree<T: ForLazySegtree> {
    original_size: usize,
    leaf_size: usize,
    node: Vec<T::M>,
    lazy: Vec<T::L>,
}
#[snippet("LazySegtree")]
impl<T: ForLazySegtree> LazySegtree<T> {
    pub fn new(n: usize) -> Self {
        let size = n.next_power_of_two();
        LazySegtree {
            original_size: n,
            leaf_size: size,
            node: vec![T::M::e(); 2 * size],
            lazy: vec![T::L::e(); 2 * size],
        }
    }

    pub fn build(&mut self, vec: &[T::M]) {
        for (i, ele) in vec.iter().enumerate() {
            let idx = i + self.leaf_size;
            self.node[idx] = ele.clone();
        }
        for i in (1..self.leaf_size).rev() {
            self.node[i] = self.node[i << 1].op(&self.node[i << 1 | 1])
        }
    }
    pub fn update(&mut self, i: usize, x: T::M) {
        //! 一点更新 O(log N)
        assert!(i < self.original_size);
        let i = i + self.leaf_size;
        self.propagate_above(i);
        self.node[i] = x;
        self.lazy[i] = T::L::e();
        self.recalc_above(i);
    }
    pub fn fold<R>(&mut self, range: R) -> T::M
    where
        R: std::ops::RangeBounds<usize>,
    {
        //! 区間取得 O(log N)
        let (l, r) = range_to_tuple(range, self.original_size);
        assert!(l < self.original_size);
        assert!(r <= self.original_size);

        let mut l = l + self.leaf_size;
        let mut r = r + self.leaf_size;

        self.propagate_above(l / (l & (!l + 1)));
        self.propagate_above((r / (r & (!r + 1))) - 1);
        // self.propagate_above(l >> l.trailing_zeros());
        // self.propagate_above(r >> r.trailing_zeros()-1);
        let mut vl = T::M::e();
        let mut vr = T::M::e();
        while l < r {
            if l & 1 == 1 {
                vl = vl.op(&self.eval_at(l));
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                vr = self.eval_at(r).op(&vr);
            }
            l >>= 1;
            r >>= 1;
        }
        vl.op(&vr)
    }
    pub fn apply_range<R>(&mut self, range: R, a: T::L)
    where
        R: std::ops::RangeBounds<usize>,
    {
        //! 区間作用 O(log N)
        let (l, r) = range_to_tuple(range, self.original_size);
        let mut l = l + self.leaf_size;
        let mut r = r + self.leaf_size;
        let l0 = l / (l & (!l + 1));
        let r0 = (r / (r & (!r + 1))) - 1;

        self.propagate_above(l0);
        self.propagate_above(r0);

        while l < r {
            if l & 1 == 1 {
                self.lazy[l] = T::L::op(&self.lazy[l], &a);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                self.lazy[r] = T::L::op(&self.lazy[r], &a);
            }
            l >>= 1;
            r >>= 1;
        }
        self.recalc_above(l0);
        self.recalc_above(r0);
    }
    pub fn max_right<F>(&mut self, l: usize, f: F) -> usize
    where
        F: Fn(&T::M) -> bool,
    {
        //! `f(l..r) == true`となる最大のrを探索
        assert!(l <= self.original_size);
        assert!(f(&T::M::e()));

        let mut l = l + self.leaf_size;
        let mut v = T::M::e();
        self.propagate_above(l / (l & (!l + 1)));

        loop {
            // 右ノードになるまで親に移動
            while l & 1 != 1 {
                l >>= 1;
            }
            // ここまでの間で、始めてfalseになるrがあるので、そこを探索
            // lの部分木の葉に着くのが目的
            if !f(&v.op(&self.node[l])) {
                while l < self.leaf_size {
                    self.propagate_at(l);
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

    pub fn min_left<F>(&mut self, r: usize, f: F) -> usize
    where
        F: Fn(&T::M) -> bool,
    {
        //! `f(l..r) == true`となる最小のlを探索
        assert!(r <= self.original_size);
        assert!(f(&T::M::e()));

        let mut r = r + self.leaf_size;
        let mut v = T::M::e();
        self.propagate_above((r / (r & (!r + 1))) - 1);

        loop {
            r -= 1;
            // 左ノードになるまで
            while r > 1 && r & 1 == 1 {
                r >>= 1;
            }
            if !f(&self.node[r].op(&v)) {
                while r < self.leaf_size {
                    self.propagate_at(r);
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

#[snippet("LazySegtree")]
impl<T: ForLazySegtree> LazySegtree<T> {
    fn eval_at(&mut self, i: usize) -> T::M {
        //! node[i]にlazy[i]を作用
        T::mapping(&self.node[i], &self.lazy[i])
    }
    fn propagate_at(&mut self, i: usize) {
        //! lazyの合成
        //! 非可換を想定しているので、lazy[i]にあるLをnode[i]に作用させる
        self.node[i] = self.eval_at(i);
        // i<<1にiから合成
        self.lazy[i << 1] = T::L::op(&self.lazy[i << 1], &self.lazy[i]);
        // i<<1|1にiから合成
        self.lazy[i << 1 | 1] = T::L::op(&self.lazy[i << 1 | 1], &self.lazy[i]);
        self.lazy[i] = T::L::e();
    }
    // iより上ののLazyを合成
    // i == 8のとき、1, 2, 4の順
    fn propagate_above(&mut self, i: usize) {
        // let h = self.exp;
        let h = std::usize::MAX.count_ones() - i.leading_zeros();
        for k in (1..h).rev() {
            self.propagate_at(i >> k);
        }
    }
    // 上部のnodeを計算
    fn recalc_above(&mut self, i: usize) {
        let mut i = i;
        while i > 1 {
            i >>= 1;
            self.node[i] = T::M::op(&self.eval_at(i << 1), &self.eval_at(i << 1 | 1))
        }
    }
}
