use std::ops::Neg;
use ::*;

impl Neg for Expr {
    type Output = Expr;
    fn neg(self) -> Expr {
        self * -1
    }
}

#[test]
fn expr() {
    let negated = -s!(x);
    assert_eq!(format!("{:?}", negated), "Mul(Symbol(\"x\"), Num(-1.0))");
}
