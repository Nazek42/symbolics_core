use ::*;

impl Expr {
    /// Square root.
    ///
    /// Provided as a convenience; `expr.sqrt()` is equivalent to `expr ^ 0.5`.
    pub fn sqrt(self) -> Expr {
        self ^ 0.5
    }
    /// Raise to the second power.
    ///
    /// Provided as a convenience; `expr.squared()` is equivalent to `expr ^ 2`.
    pub fn squared(self) -> Expr {
        self ^ 2
    }
    /// Raise to the third power.
    ///
    /// Provided as a convenience; `expr.cubed()` is equivalent to `expr ^ 3`.
    pub fn cubed(self) -> Expr {
        self ^ 3
    }
}
