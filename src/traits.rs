use cargo_snippet::snippet;

#[snippet("Monoid")]
pub trait Monoid {
    /// モノイドの二項演算
    fn op(&self, right: &Self) -> Self;
    /// 二項演算の単位元
    fn e() -> Self;
}

