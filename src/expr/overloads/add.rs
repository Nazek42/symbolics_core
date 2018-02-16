use std::ops::Add;
use ::*;

impl <T> Add<T> for Expr where T: Into<Expr> {
    type Output = Expr;
    fn add(self, rhs: T) -> Expr {
        Expr::Add(Box::new(self.clone()), Box::new(rhs.into()))
    }
}

impl Add<Expr> for f64 {
    type Output = Expr;
    fn add(self, rhs: Expr) -> Expr {
        Expr::Num(self) + rhs
    }
}

impl Add<Expr> for i64 {
    type Output = Expr;
    fn add(self, rhs: Expr) -> Expr {
        Expr::Num(self as f64) + rhs
    }
}

#[test]
fn expr_f64() {
    let added = s!(x) + 3;
    assert_eq!(format!("{:?}", added), "Add(Symbol(\"x\"), Num(3.0))");
}

#[test]
fn f64_expr() {
    let added = 3 + s!(x);
    assert_eq!(format!("{:?}", added), "Add(Num(3.0), Symbol(\"x\"))");
}

#[test]
fn expr_expr() {
    let added = s!(x) + s!(y);
    assert_eq!(format!("{:?}", added), "Add(Symbol(\"x\"), Symbol(\"y\"))");
}
