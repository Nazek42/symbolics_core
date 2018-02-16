use std::ops::BitXor;
use ::*;

impl <T> BitXor<T> for Expr where T: Into<Expr> {
    type Output = Expr;
    fn bitxor(self, rhs: T) -> Expr {
        Expr::Pow(Box::new(self.clone()), Box::new(rhs.into()))
    }
}

impl BitXor<Expr> for f64 {
    type Output = Expr;
    fn bitxor(self, rhs: Expr) -> Expr {
        Expr::Num(self) ^ rhs.clone()
    }
}

impl BitXor<Expr> for i64 {
    type Output = Expr;
    fn bitxor(self, rhs: Expr) -> Expr {
        Expr::Num(self as f64) ^ rhs.clone()
    }
}

#[test]
fn expr_f64() {
    let expd = s!(x)^2;
    assert_eq!(format!("{:?}", expd), "Pow(Symbol(\"x\"), Num(2.0))");
}

#[test]
fn f64_expr() {
    let expd = 10^s!(x);
    assert_eq!(format!("{:?}", expd), "Pow(Num(10.0), Symbol(\"x\"))");
}

#[test]
fn expr_expr() {
    let expd = s!(x) ^ s!(y);
    assert_eq!(format!("{:?}", expd), "Pow(Symbol(\"x\"), Symbol(\"y\"))");
}
