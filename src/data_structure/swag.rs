// use cargo_snippet::snippet;

use crate::traits::Monoid;

// #[snippet("Swag")]
/// 非可換な演算もできるswag
#[derive(Debug)]
pub struct Swag<T>
where
    T: Monoid,
{
    front: Vec<T::S>,
    back: Vec<T::S>,
    back_raw: Vec<T::S>,
}
// #[snippet("Swag")]

impl<T> Swag<T>
where
    T: Monoid,
{
    pub fn new() -> Self {
        Swag {
            front: vec![],
            back: vec![],
            back_raw: vec![],
        }
    }
    fn add_front(&mut self, x: T::S) {
        if self.front.is_empty() {
            self.front.push(x);
        } else {
            let vi = &self.front[self.front.len() - 1];
            self.front.push(T::op(&x, &vi));
        }
    }
    fn add_back(&mut self, x: T::S) {
        if self.back.is_empty() {
            self.back.push(x);
        } else {
            let vi = &self.back[self.back.len() - 1];
            self.back.push(T::op(&vi, &x));
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
    pub fn push_back(&mut self, x: T::S) {
        self.add_back(x.clone());
        self.back_raw.push(x);
    }
    pub fn pop_front(&mut self) -> Option<T::S> {
        self.b_to_f();
        self.front.pop()
    }
    pub fn fold(&mut self) -> T::S {
        if self.front.is_empty() && self.back.is_empty() {
            return T::e();
        }
        if self.front.is_empty() {
            return self.back[self.back.len() - 1].clone();
        }
        if self.back.is_empty() {
            return self.front[self.front.len() - 1].clone();
        }
        // self.front[self.front.len() - 1].op(&self.back[self.back.len() - 1])
        T::op(
            &self.front[self.front.len() - 1],
            &self.back[self.back.len() - 1],
        )
    }
}
// #[snippet("Swag")]
impl<T> Default for Swag<T>
where
    T: Monoid + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{data_structure::swag::Swag, traits::Monoid};

    #[test]
    fn it_works() {
        #[derive(Debug, Clone, Copy)]
        struct Min {}
        impl Monoid for Min {
            type S = usize;
            fn e() -> Self::S {
                0
            }
            fn op(left: &Self::S, right: &Self::S) -> Self::S {
                left.min(right).clone()
            }
        }
        let mut swag = Swag::<Min>::new();
        let v = (0..6).collect::<Vec<usize>>();
        for &i in &v {
            swag.push_back(i);
            println!("{:?}", &swag.front);
            println!("{:?}", &swag.back);
            println!("{:?}", &swag.back_raw);
            println!("{}", swag.fold());
        }
        println!();
        println!("POP");
        for _ in 0..6 {
            println!("{}", swag.pop_front().unwrap());
            println!("{:?}", &swag.front);
            println!("{}", swag.fold());
        }

        let mut swag = Swag::<Min>::new();
        let v = (0..6).collect::<Vec<usize>>();
        for i in v {
            swag.push_back(i);
        }
        assert_eq!(0, swag.fold());
        assert_eq!(Some(0), swag.pop_front());
        assert_eq!(1, swag.fold());
        assert_eq!(Some(1), swag.pop_front());
        assert_eq!(2, swag.fold());
    }
}
