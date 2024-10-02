use cargo_snippet::snippet;

use crate::traits::CommutaitveGroup;

pub struct PotentialedUnionFind {
    len: usize,
    par: Vec<usize>,
    size: Vec<usize>,
    diff_weight: Vec<i64>,
}
impl PotentialedUnionFind {
    pub fn new(n: usize) -> Self {
        PotentialedUnionFind {
            len: n,
            par: (0..n).collect(),
            size: vec![0; n],
            diff_weight: vec![0; n],
        }
    }
    pub fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            let r = self.root(self.par[x]);
            self.diff_weight[x] += self.diff_weight[self.par[x]];
            self.par[x] = r;
            r
        }
    }
    pub fn weight(&mut self, x: usize) -> i64 {
        self.root(x);
        self.diff_weight[x]
    }
    pub fn diff(&mut self, x: usize, y: usize) -> i64 {
        //! `x-y`
        self.weight(x) - self.weight(y)
    }
    pub fn unite(&mut self, x: usize, y: usize, w: i64) -> bool {
        //! `weight(y) - weight(x) == w`となるようにunite
        //! 
        //! 正しく辺が張れるなら`true`そうでないなら`false`を返す
        let mut w = w;
        w += self.weight(x);
        w -= self.weight(y);

        let mut x = self.root(x);
        let mut y = self.root(y);

        if x == y {
            return w == 0;
        }

        if self.size[x] < self.size[y] {
            std::mem::swap(&mut x, &mut y);
            w = -w;
        }
        self.par[y] = x;
        self.size[x] += self.size[y];
        self.diff_weight[y] = w;

        true
    }
    pub fn size(&mut self, x: usize) -> usize {
        let r = self.root(x);
        self.size[r]
    }
    pub fn issame(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    pub fn group_count(&mut self) -> usize {
        //!  `UnionFind`内の集合の数を返す
        //!
        //! `O(Nα(N)) `
        let mut cnt = vec![0; self.len];
        for i in 0..self.len {
            cnt[self.root(i)] += 1;
        }
        cnt.iter().filter(|&&i| 0 < i).count()
    }
}

pub struct AbelPotentialedUnionFind<T>
where
    T: CommutaitveGroup,
{
    len: usize,
    par: Vec<usize>,
    size: Vec<usize>,
    diff_weight: Vec<T::S>,
}
impl<T> AbelPotentialedUnionFind<T>
where
    T: CommutaitveGroup,
    T::S: Eq,
{
    pub fn new(n: usize) -> Self {
        AbelPotentialedUnionFind {
            len: n,
            par: (0..n).collect(),
            size: vec![0; n],
            diff_weight: vec![T::e(); n],
        }
    }
    pub fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            let r = self.root(self.par[x]);
            // self.diff_weight[x] = self.diff_weight[x].op(&self.diff_weight[self.par[x]]);
            self.diff_weight[x] = T::op(&self.diff_weight[x], &self.diff_weight[self.par[x]]);
            self.par[x] = r;
            r
        }
    }
    pub fn weight(&mut self, x: usize) -> T::S {
        self.root(x);
        self.diff_weight[x].clone()
    }
    pub fn diff(&mut self, x: usize, y: usize) -> T::S {
        //! `x-y`
        let x = self.weight(x);
        let y = T::inv(&self.weight(y));
        T::op(&x, &y)
    }
    pub fn unite(&mut self, x: usize, y: usize, w: T::S) -> bool {
        //! `weight(y) - weight(x) == w`となるようにunite
        //! 
        //! 正しく辺が張れるなら`true`そうでないなら`false`を返す
        let mut w = w;
        // w = w.op(&self.weight(x));
        w = T::op(&w, &self.weight(x));
        // w = w.op(&self.weight(y).inv());
        w = T::op(&w, &T::inv(&self.weight(y)));

        let mut x = self.root(x);
        let mut y = self.root(y);
        if x == y {
            return w == T::e();
        }
        if self.size[x] < self.size[y] {
            std::mem::swap(&mut x, &mut y);
            w = T::inv(&w);
        }
        self.par[y] = x;
        self.size[x] += self.size[y];
        self.diff_weight[y] = w;

        true
    }
    pub fn size(&mut self, x: usize) -> usize {
        let r = self.root(x);
        self.size[r]
    }
    pub fn issame(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    pub fn group_count(&mut self) -> usize {
        //!  `UnionFind`内の集合の数を返す
        //!
        //! `O(Nα(N)) `
        let mut cnt = vec![0; self.len];
        for i in 0..self.len {
            cnt[self.root(i)] += 1;
        }
        cnt.iter().filter(|&&i| 0 < i).count()
    }
}

#[snippet("UnionFind")]
pub struct UnionFind {
    len: usize,
    par: Vec<usize>,
    size: Vec<usize>,
    // edgecount: Vec<usize>,
}
#[snippet("UnionFind")]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            len: n,
            par: (0..n).collect(),
            size: vec![1; n],
            // edgecount: vec![0; n],
        }
    }

    pub fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            self.par[x] = self.root(self.par[x]);
            self.par[x]
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.root(x);
        let mut y = self.root(y);
        if x == y {
            return false;
        }
        if self.size[x] < self.size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.par[y] = x;
        self.size[x] += self.size[y];
        true
    }

    pub fn issame(&mut self, x: usize, y: usize) -> bool {
        if self.root(x) == self.root(y) {
            return true;
        }
        false
    }

    pub fn size(&mut self, x: usize) -> usize {
        let r = self.root(x);
        self.size[r]
    }

    // pub fn edge(&mut self, x: usize) -> usize {
    //     let r = self.root(x);
    //     self.edgecount[r]
    // }

    pub fn group_count(&mut self) -> usize {
        //!  `UnionFind`内の集合の数を返す
        //!
        //! `O(Nα(N)) `
        let mut cnt = vec![0; self.len];
        for i in 0..self.len {
            cnt[self.root(i)] += 1;
        }
        cnt.iter().filter(|&&i| 0 < i).count()
    }
}

#[cfg(test)]
mod test {}
