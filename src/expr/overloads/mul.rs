use std::ops::{Mul, MulAssign};
use ::*;

impl <T> Mul<T> for Expr where T: Into<Expr>{
    type Output = Expr;
    fn mul(self, rhs: T) -> Expr {
        Expr::Mul(Box::new(self.clone()), Box::new(rhs.into()))
    }
}

impl <T> MulAssign<T> for Expr where T: Into<Expr> {
    fn mul_assign(&mut self, rhs: T) {
        *self = self.clone() * rhs.into()
    }
}

impl Mul<Expr> for f64 {
    type Output = Expr;
    fn mul(self, rhs: Expr) -> Expr {
        Expr::Num(self) * rhs.clone()
    }
}

impl Mul<Expr> for i64 {
    type Output = Expr;
    fn mul(self, rhs: Expr) -> Expr {
        Expr::Num(self as f64) * rhs.clone()
    }
}

#[test]
fn expr_f64() {
    let added = s!(x) * 3;
    assert_eq!(format!("{:?}", added), "Mul(Symbol(\"x\"), Num(3.0))");
}

#[test]
fn f64_expr() {
    let added = 3*s!(x);
    assert_eq!(format!("{:?}", added), "Mul(Num(3.0), Symbol(\"x\"))");
}

#[test]
fn expr_expr() {
    let added = s!(x) * s!(y);
    assert_eq!(format!("{:?}", added), "Mul(Symbol(\"x\"), Symbol(\"y\"))");
}
