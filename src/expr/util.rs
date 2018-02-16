use std::ops::Not;
use std::f64::consts;
use ::*;

impl Expr {
    /// Return whether the expression represents a literal number,
    /// i.e. whether it is of variant `Num`.
    pub fn is_num(self) -> bool {
        match self {
            Expr::Num(_) => true,
            _ => false
        }
    }
    /// Return the numeric value of the expression, if all symbols have been applied.
    /// If there are unapplied symbols, returns `None`.
    pub fn val(self) -> Option<f64> {
        match self.apply1("$e", consts::E).apply1("$pi", consts::PI) {
            Expr::Num(x) => Some(x),
            _ => None
        }
    }
}

impl From<f64> for Expr {
    fn from(x: f64) -> Expr {
        Expr::Num(x)
    }
}

impl From<i64> for Expr {
    fn from(x: i64) -> Expr {
        Expr::Num(x as f64)
    }
}

// This allows !&x syntax. Ugly, but it might be useful.
impl<'a> Not for &'a Expr {
    type Output = Expr;
    fn not(self) -> Expr {
        self.clone()
    }
}
