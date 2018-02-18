#[macro_export]
/// Produce a symbol with the name `name`. This is preferred over explicitly declaring symbols.
///
/// # Examples
///
/// ```
/// # #[macro_use]
/// # extern crate symbolics_core;
/// # fn main() {
///       let x = s!(x);
///       assert_eq!(format!("{:?}", x), "Symbol(\"x\")");
/// # }
/// ```
macro_rules! s {
    ($name:ident) => {$crate::Expr::Symbol(stringify!($name).to_owned())}
}

#[macro_export]
/// Apply any number of substitutions to an expression. These can be numbers or expressions.
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate symbolics_core;
/// fn main() {
///     let expr = s!(x) * s!(x).sqrt();
///     assert_eq!(apply!(expr, x=4.).val().unwrap(), 8.);
/// }
/// ```
///
/// ```
/// #[macro_use]
/// extern crate symbolics_core;
/// use std::f64::consts::E;
/// fn main() {
///     let expr = s!(x) + s!(x).sqrt();
///     let expr_t = apply!(expr, x = s!(t).ln());
///     assert_eq!(apply!(expr_t, t = E).val().unwrap(), 2.);
/// }
/// ```
macro_rules! apply {
    ($expr:expr, $($sym:ident = $sub:expr),+) => {
        $expr $(.apply1(stringify!($sym), $sub))+
    }
}
