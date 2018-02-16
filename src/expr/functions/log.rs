use ::*;

impl Expr {
    /// Logarithm of arbitrary base.
    pub fn log<T>(self, base: T) -> Expr where T: Into<Expr> {
        Expr::Log(Box::new(self.clone()), Box::new(base.into()))
    }
    /// Logarithm with base 2.
    pub fn log2(self) -> Expr {
        self.log(2)
    }
    /// Logarithm with base 10.
    pub fn log10(self) -> Expr {
        self.log(10)
    }
    /// Natural logarithm, i.e. log base *e*, where *e* is Euler's number.
    pub fn ln(self) -> Expr {
        self.log(consts::e())
    }
}
