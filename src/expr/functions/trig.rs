use ::*;

impl Expr {
    pub fn sin(self) -> Expr {
        Expr::Sin(Box::new(self.clone()))
    }
    pub fn cos(self) -> Expr {
        Expr::Cos(Box::new(self.clone()))
    }
    pub fn tan(self) -> Expr {
        self.clone().sin() / self.cos()
    }
    pub fn csc(self) -> Expr {
        1. / self.sin()
    }
    pub fn sec(self) -> Expr {
        1. / self.cos()
    }
    pub fn cot(self) -> Expr {
        self.clone().cos() / self.sin()
    }
    pub fn asin(self) -> Expr {
        Expr::Arcsin(Box::new(self.clone()))
    }
    pub fn acos(self) -> Expr {
        Expr::Arccos(Box::new(self.clone()))
    }
    pub fn atan(self) -> Expr {
        Expr::Arctan(Box::new(self.clone()))
    }
}
