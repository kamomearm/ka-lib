use crate::traits::Monoid;

#[derive(Debug, Clone, Copy)]
struct ZobHshP {
    hash: u128
}
impl Monoid for ZobHshP {
    fn op(&self, right: &ZobHshP) -> ZobHshP {
        let l = self.hash;
        let r = right.hash;
        ZobHshP { hash: l+r }
    }
    fn e() -> ZobHshP {
        ZobHshP { hash: 0 }
    }
}