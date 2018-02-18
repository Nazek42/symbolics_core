#[derive(Debug, Clone)]
pub enum Expr {
    Num(f64),
    Symbol(String),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Log(Box<Expr>, Box<Expr>),
    Sin(Box<Expr>),
    Cos(Box<Expr>),
    Arcsin(Box<Expr>),
    Arccos(Box<Expr>),
    Arctan(Box<Expr>),
}

mod util;
mod apply;
mod overloads;
mod functions;
