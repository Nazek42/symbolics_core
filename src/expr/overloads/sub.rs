use std::ops::Sub;
use ::*;

impl <T> Sub<T> for Expr where T: Into<Expr> {
    type Output = Expr;
    fn sub(self, rhs: T) -> Expr {
        self + (-rhs.into())
    }
}

impl Sub<Expr> for f64 {
    type Output = Expr;
    fn sub(self, rhs: Expr) -> Expr {
        Expr::Num(self) - rhs
    }
}

impl Sub<Expr> for i64 {
    type Output = Expr;
    fn sub(self, rhs: Expr) -> Expr {
        Expr::Num(self as f64) - rhs
    }
}

#[test]
fn expr_f64() {
    let subbed = s!(x) - 3;
    assert_eq!(format!("{:?}", subbed), "Add(Symbol(\"x\"), Mul(Num(3.0), Num(-1.0)))");
}

#[test]
fn f64_expr() {
    let subbed = 3 - s!(x);
    assert_eq!(format!("{:?}", subbed), "Add(Num(3.0), Mul(Symbol(\"x\"), Num(-1.0)))");
}

#[test]
fn expr_expr() {
    let subbed = s!(x) - s!(y);
    assert_eq!(format!("{:?}", subbed), "Add(Symbol(\"x\"), Mul(Symbol(\"y\"), Num(-1.0)))");
}
