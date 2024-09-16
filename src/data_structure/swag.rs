use crate::algebra::Monoid;
use cargo_snippet::snippet;

#[snippet("Swag")]
/// 非可換な演算もできるswag
#[derive(Debug)]
pub struct Swag<T>
where
    T: Monoid + Clone,
{
    front: Vec<T>,
    back: Vec<T>,
    back_raw: Vec<T>,
}
#[snippet("Swag")]

impl<T> Swag<T>
where
    T: Monoid + Clone,
{
    pub fn new() -> Self {
        Swag {
            front: vec![],
            back: vec![],
            back_raw: vec![],
        }
    }
    fn add_front(&mut self, x: T) {
        if self.front.is_empty() {
            self.front.push(x);
        } else {
            let vi = &self.front[self.front.len() - 1];
            self.front.push(x.op(&vi));
        }
    }
    fn add_back(&mut self, x: T) {
        if self.back.is_empty() {
            self.back.push(x);
        } else {
            let vi = &self.back[self.back.len() - 1];
            self.back.push(vi.op(&x));
        }
    }
    fn b_to_f(&mut self) {
        if self.front.is_empty() {
            while self.back.pop().is_some() {
                let ri = self.back_raw.pop().unwrap();
                self.add_front(ri);
            }
        }
    }
    pub fn push_back(&mut self, x: T) {
        self.add_back(x.clone());
        self.back_raw.push(x);
    }
    pub fn pop_front(&mut self) -> Option<T> {
        self.b_to_f();
        self.front.pop()
    }
    pub fn fold(&mut self) -> T {
        if self.front.is_empty() && self.back.is_empty() {
            return T::e();
        }
        if self.front.is_empty() {
            return self.back[self.back.len() - 1].clone();
        }
        if self.back.is_empty() {
            return self.front[self.front.len() - 1].clone();
        }
        self.front[self.front.len() - 1].op(&self.back[self.back.len() - 1])
    }
}
#[cfg(test)]
mod tests {
    use crate::{algebra::Monoid, data_structure::swag::Swag};

    #[test]
    fn it_works() {
        #[derive(Debug, Clone, Copy)]
        struct Min {
            val: usize
        }
        impl Monoid for Min {
            fn op(&self, right: &Min) -> Min {
                Min { val: self.val.min(right.val) }
            }
            fn e() -> Min {
                Min { val: usize::MAX }
            }
        }
        let mut swag = Swag::<Min>::new();
        let v = (0..6).map(|i| Min {val: i}).collect::<Vec<Min>>();
        for i in &v {
            swag.push_back(i.clone());
            println!("{:?}", &swag.front);
            println!("{:?}", &swag.back);
            println!("{:?}", &swag.back_raw);
            println!("{}", swag.fold().val);
        }
        println!();
        println!("POP");
        for _ in 0..6 {
            println!("{}", swag.pop_front().unwrap().val);
            println!("{:?}", &swag.front);
            println!("{}", swag.fold().val);
        }
    }
}
