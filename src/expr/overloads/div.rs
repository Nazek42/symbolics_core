use std::ops::Div;
use ::*;

impl <T> Div<T> for Expr where T: Into<Expr>{
    type Output = Expr;
    fn div(self, rhs: T) -> Expr {
        self * (rhs.into() ^ -1.)
    }
}

impl Div<Expr> for f64 {
    type Output = Expr;
    fn div(self, rhs: Expr) -> Expr {
        Expr::Num(self) * (rhs.clone() ^ -1.)
    }
}

impl Div<Expr> for i64 {
    type Output = Expr;
    fn div(self, rhs: Expr) -> Expr {
        Expr::Num(self as f64) * (rhs.clone() ^ -1.)
    }
}

#[test]
fn expr_f64() {
    let added = s!(x) / 3;
    assert_eq!(format!("{:?}", added), "Mul(Symbol(\"x\"), Pow(Num(3.0), Num(-1.0)))");
}

#[test]
fn f64_expr() {
    let added = 3 / s!(x);
    assert_eq!(format!("{:?}", added), "Mul(Num(3.0), Pow(Symbol(\"x\"), Num(-1.0)))");
}

#[test]
fn expr_expr() {
    let added = s!(x) / s!(y);
    assert_eq!(format!("{:?}", added), "Mul(Symbol(\"x\"), Pow(Symbol(\"y\"), Num(-1.0)))");
}
