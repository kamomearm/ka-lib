use cargo_snippet::snippet;

#[snippet]
pub struct UnionFind {
    v_size: usize,
    par: Vec<i64>,
    siz: Vec<usize>,
    edgecount: Vec<usize>,
}
#[snippet("UnionFind")]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            v_size: n,
            par: vec![-1; n],
            siz: vec![1; n],
            edgecount: vec![0; n],
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
        let rx = self.root(x);
        let ry = self.root(y);
        if rx == ry {
            self.edgecount[rx] += 1;
            return false;
        }
        if self.siz[rx] < self.siz[ry] {
            self.par[rx] = ry as i64;
            self.siz[ry] += self.siz[rx];
            self.edgecount[ry] += 1 + self.edgecount[rx];
        } else {
            self.par[ry] = rx as i64;
            self.siz[rx] += self.siz[ry];
            self.edgecount[rx] += 1 + self.edgecount[ry];
        }
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
        self.siz[r]
    }

    pub fn edge(&mut self, x: usize) -> usize {
        let r = self.root(x);
        self.edgecount[r]
    }
    
    pub fn groupcout(&mut self) -> usize {
        use std::collections::HashMap;
        let mut dic: HashMap<usize, usize> = HashMap::new();
        for i in 0..self.v_size {
            let r = self.root(i);
            *dic.entry(r).or_default() += 1;
        }
        let mut ret = 0;
        for value in dic.values() {
            ret = ret.max(*value);
        }
        ret
    }
}

#[cfg(test)]
mod test {

}