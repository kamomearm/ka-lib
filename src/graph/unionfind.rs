use cargo_snippet::snippet;

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
        //! `x`と`y`の差
        self.weight(x) - self.weight(y)
    }
    pub fn unite(&mut self, x: usize, y: usize, w: i64) -> bool {
        //! `weight(y) - weight(x) == w`となるようにunite
        let mut w = w;
        w += self.weight(x);
        w -= self.weight(y);

        let mut x = self.root(x);
        let mut y = self.root(y);
        if x == y {
            return false;
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
}

#[snippet("UnionFind")]
pub struct UnionFind {
    len: usize,
    par: Vec<i64>,
    size: Vec<usize>,
    // edgecount: Vec<usize>,
}
#[snippet("UnionFind")]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            len: n,
            par: vec![-1; n],
            size: vec![1; n],
            // edgecount: vec![0; n],
        }
    }

    pub fn root(&mut self, x: usize) -> usize {
        if self.par[x] == -1 {
            x
        } else {
            self.par[x] = self.root(self.par[x] as usize) as i64;
            self.par[x] as usize
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
        self.par[y] = x as i64;
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
