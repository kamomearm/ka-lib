use cargo_snippet::snippet;

#[snippet("CompMap")]
#[derive(Debug)]
pub struct CompMap<T> {
    original: Vec<T>
}
#[snippet("CompMap")]
impl <T> CompMap<T> 
where T: Clone+Ord {
    
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
    
        while ok-ng > 1 {
            let mid = (ok+ng)/2;
            if *x <= self.original[mid as usize] {
                ok = mid as isize;
            }
            else {
                ng = mid as isize;
            }
        }
        ok as usize
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        // abc036
        
    }
}
