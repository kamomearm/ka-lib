use cargo_snippet::snippet;

/// 位置`p`から`k`ターン先の行き先を求める
pub trait Doubling {
    type D;
    fn new(v: &[Self::D], k: usize) -> Self;
    fn query(&self, k: usize, idx: usize) -> Self::D;
    fn set_next(&mut self, x: Self::D, idx: usize);
}

// #[snippet("Monoid")]
// pub trait Monoid {
//     /// モノイドの二項演算
//     fn op(&self, right: &Self) -> Self;
//     /// 二項演算の単位元
//     fn e() -> Self;
// }

#[snippet("Monoid")]
pub trait Monoid {
    type S: Clone;
    fn op(left: &Self::S, right: &Self::S) -> Self::S;
    fn e() -> Self::S;
}

// pub trait Biop {
//     /// 二項演算
//     fn op(&self, right: &Self) -> Self;
// }

pub trait E {
    /// 二項演算の単位元
    fn e() -> Self;
}
pub trait Inv: Monoid {
    /// 二項演算の逆元
    fn inv(s: &Self::S) -> Self::S;
}
pub trait Commutative {}

pub trait CommutaitveGroup: Monoid + Commutative + Inv {}
pub trait Group: Monoid + Inv {}
// pub trait TMonoid: Biop + E {}
// pub trait SemiGroup: Biop {}
