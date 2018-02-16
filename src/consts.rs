use ::*;

/// Create a symbol representing the constant 𝑒.
///
/// This will be exact when evaluated, so it is preferred over `std::f64::consts::E`.
pub fn e() -> Expr {
    Expr::Symbol("$e".to_owned())
}
/// Create a symbol representing the constant 𝜋.
///
/// This will be exact when evaluated, so it is preferred over `std::f64::consts::PI`.
pub fn pi() -> Expr {
    Expr::Symbol("$pi".to_owned())
}
