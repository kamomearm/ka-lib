#[cfg(test)]
mod tests {
    use marker::{Chars, Usize1};
    use proconio::*;

    use crate::str_algo::rolling_hash::RollingHsh;
    #[test]
    fn it_works() {
        let v = vec!['a', 'b', 'c', 'a', 'b', 'c'];
        let r = RollingHsh::new(&v);
        let a = r.get_hash(0..=2);
        let b = r.get_hash(3..=5);
        assert_eq!(a, b);
    }
    fn tessoku_a56() {
        input! {
            n: usize,
            q: usize,
            s: Chars
        }
        let rollinghash = RollingHsh::new(&s);
        for _ in 0..q {
            input! {
                a: Usize1, b: usize, c: Usize1, d: usize
            }
            if rollinghash.get_hash(a..b) == rollinghash.get_hash(c..d) {
                println!("Yes")
            }
            else {
                println!("No")
            }
        }
    }
}


pub struct RollingHsh {
    modulo: i128,
    hash: Vec<i128>,
    base_pow: Vec<i128>
}

impl RollingHsh {
    pub fn new(v: &Vec<char>) -> Self {
        use rand::Rng;
        let modulo: i128 = (1<<61) -1 ;

        let mut rng = rand_pcg::Pcg64Mcg::new(0);
        let base = rng.gen_range(2..=modulo-2);

        let mut base_pow = vec![1; v.len()+1];
        for i in 0..v.len() {
            base_pow[i+1] = (base_pow[i]*base)%modulo;
        }

        let mut hash = vec![0; v.len()+1];
        for i in 0..v.len() {
            hash[i+1] = ((hash[i]*base)%modulo + v[i] as i128)%modulo;
        }

        RollingHsh { modulo, hash, base_pow }
    }
    pub fn get_hash<R>(&self, range: R) -> i128
    where 
        R: std::ops::RangeBounds<usize>
    {
        //! 与えられた区間の`Hash`値を返す
        // l..r
        let (l, r) = Self::range_to_tuple(range, self.hash.len()-1);
        // 1..2
        // 0, h0*base^0, h0*base^1 + h1*base^0  
        let r_hash = self.hash[r];
        let l_hash = self.hash[l];
        // eprintln!("{}", r_hash);
        // eprintln!("{}", l_hash*self.base_pow[r-l]);
        (r_hash - l_hash*self.base_pow[r-l]).rem_euclid(self.modulo)
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
}

