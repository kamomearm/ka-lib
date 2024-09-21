use crate::traits::Monoid;

/// 位置`p`から`k`ターン先の行き先を求める
pub trait Doubling {
    type D;
    fn new(v: &[Self::D], k: usize) -> Self;
    fn query(&self, k: usize, idx: usize) -> Self::D;
    fn set_next(&mut self, x: Self::D, idx: usize);
}

pub struct WeightDoubling<T> {
    table: Vec<Vec<Option<(usize, T)>>>,
}
impl<M> Doubling for WeightDoubling<M>
where
    M: Monoid + Clone,
{
    type D = Option<(usize, M)>;
    fn new(v: &[Self::D], k: usize) -> Self {
        //! `ターンの最大値 <= 2^kとなるk`
        let mut table = vec![vec![None; v.len()]; k];
        for (i, val) in v.iter().enumerate() {
            table[0][i] = val.clone();
        }

        for k in 0..k - 1 {
            for i in 0..v.len() {
                match &table[k][i] {
                    Some((p, m)) => {
                        let nval = table[k][*p].clone();
                        match nval {
                            Some((np, nm)) => table[k + 1][i] = Some((np, m.op(&nm))),
                            None => table[k + 1][i] = None,
                        }
                    }
                    None => table[k + 1][i] = None,
                }
            }
        }
        WeightDoubling { table }
    }
    fn query(&self, k: usize, idx: usize) -> Self::D {
        let mut m = M::e();
        let mut x = idx;
        for i in 0..=self.table.len() {
            if k >> i & 1 == 1 {
                match &self.table[i][x] {
                    Some((nx, nm)) => {
                        x = nx.clone();
                        m = m.op(nm);
                    }
                    None => return None,
                }
            }
        }
        Some((x, m))
    }
    fn set_next(&mut self, x: Self::D, idx: usize) {
        self.table[0][idx] = x;
    }
}

pub struct UnWeightDoubling {
    table: Vec<Vec<Option<usize>>>,
}
impl Doubling for UnWeightDoubling {
    type D = Option<usize>;

    fn new(v: &[Self::D], k: usize) -> Self {
        //! `ターンの最大値 <= 2^kとなるk`
        let mut table = vec![vec![None; v.len()]; k];

        for (i, val) in v.iter().enumerate() {
            table[0][i] = val.clone();
        }

        for k in 0..k - 1 {
            for i in 0..v.len() {
                table[k][i].map(|val| table[k + 1][i] = table[k][val]);
            }
        }

        UnWeightDoubling { table }
    }
    fn query(&self, k: usize, idx: usize) -> Self::D {
        let mut x = idx;
        for i in 0..=self.table.len() {
            if k >> i & 1 == 1 {
                match self.table[i][x] {
                    Some(nx) => x = nx,
                    None => return None,
                }
            }
        }
        Some(x)
    }
    fn set_next(&mut self, x: Self::D, idx: usize) {
        self.table[0][idx] = x;
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn it_work() {}
}
