use std::ops::Neg;
use ::*;

impl Neg for Expr {
    type Output = Expr;
    fn neg(self) -> Expr {
        Expr::Neg(Box::new(self.clone()))
    }
}

#[test]
fn expr() {
    let negated = -s!(x);
    assert_eq!(format!("{:?}", negated), "Neg(Symbol(\"x\"))");
}
