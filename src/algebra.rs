use crate::traits::Monoid;

#[derive(Debug, Clone, Copy)]
struct ZobHshP;
impl Monoid for ZobHshP {
    fn e() -> Self::S {
        0
    }
    fn op(left: &Self::S, right: &Self::S) -> Self::S {
        left + right
    }
    type S = u128;
}
