use crate::traits::{Doubling, Monoid};

pub struct WeightDoubling<T> 
where 
    T: Monoid    
{
    table: Vec<Vec<Option<(usize, T::S)>>>,
}
impl<T> Doubling for WeightDoubling<T>
where
    T: Monoid,
{
    type D = Option<(usize, T::S)>;
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
                            Some((np, nm)) => table[k + 1][i] = Some((np, T::op(&m, &nm))),
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
        let mut m = T::e();
        let mut x = idx;
        for i in 0..=self.table.len() {
            if k >> i & 1 == 1 {
                // match &self.table[i][x] {
                //     Some((nx, nm)) => {
                //         x = nx.clone();
                //         // m = m.op(nm);
                //         m = T::op(&m, &nm);
                //     }
                //     None => return None,
                // }
                let (nx, nm) = self.table[i][x]?;
                x = nx;
                m = T::op(&m, &nm);
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
                // table[k][i].map(|val| table[k + 1][i] = table[k][val]);
            }
        }

        UnWeightDoubling { table }
    }
    fn query(&self, k: usize, idx: usize) -> Self::D {
        let mut x = idx;
        for i in 0..=self.table.len() {
            if k >> i & 1 == 1 {
                // match self.table[i][x] {
                //     Some(nx) => x = nx,
                //     None => return None,
                // }
                let x = self.table[i][x]?;
            }
        }
        Some(x)
    }
    fn set_next(&mut self, x: Self::D, idx: usize) {
        self.table[0][idx] = x;
    }
}
impl UnWeightDoubling {
    pub fn query_vec(&self, k: usize) -> Vec<Option<usize>> {
        let mut ret = (0..self.table[0].len())
            .map(|i| Some(i))
            .collect::<Vec<Option<usize>>>();
        for i in 0..self.table.len() {
            if k >> i & 1 == 1 {
                ret = ret
                    .iter()
                    .map(|j| self.table[i][j.unwrap()])
                    .collect::<Vec<Option<usize>>>();
            }
        }
        ret
    }
}
#[cfg(test)]
mod test {

    #[test]
    fn it_work() {}
}
