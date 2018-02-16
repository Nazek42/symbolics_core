use ::*;

// Helper macros for the `apply1` function.
macro_rules! recursive_apply {
    ($func:ident ($($var:ident),+), $name:expr, $val:expr) => {
        $func ($(Box::new($var.apply1($name, $val))),+)
    }
}

macro_rules! simplify_nn {
    ($($var:ident),+ => $body:expr, else $else_:expr) => {
        if $($var.clone().is_num() &&)+ true {
            $(let $var = $var.val().unwrap();)+
            Num($body)
        } else {
            $else_
        }
    }
}

impl Expr {
    /// Apply a single substitution, then simplify purely numeric sub-expressions.
    /// The substitution can be anything that can be converted to an expression.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate symbolics_expressions;
    /// # fn main() {
    ///       let y = 3*s!(x).squared() + 2;
    ///       let y_2 = y.apply1("x", 2);
    ///       assert_eq!(y_2.val().unwrap(), 14.);
    /// # }
    /// ```
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate symbolics_expressions;
    /// # fn main() {
    ///       let y = 3*s!(x) + 2;
    ///       let y_t = y.apply1("x", s!(t).squared());
    ///       assert_eq!(format!("{:?}", y_t), "Add(Mul(Num(3.0), Pow(Symbol(\"t\"), Num(2.0))), Num(2.0))")
    /// # }
    /// ```
    pub fn apply1<T>(self, name: &'static str, value: T) -> Expr
    where T: Clone + Into<Expr> {
        use Expr::*;
        let partial = match self {
            Symbol(s) => if s == name {
                value.into()
            } else {
                Symbol(s.clone())
            },
            Num(n) => Num(n),
            Add(a, b) => recursive_apply!(Add(a, b), name, value.clone()),
            Mul(a, b) => recursive_apply!(Mul(a, b), name, value.clone()),
            Pow(a, b) => recursive_apply!(Pow(a, b), name, value.clone()),
            Log(a, b) => recursive_apply!(Log(a, b), name, value.clone()),
            Neg(x) => recursive_apply!(Neg(x), name, value.clone()),
            Sin(x) => recursive_apply!(Sin(x), name, value.clone()),
            Cos(x) => recursive_apply!(Cos(x), name, value.clone()),
            Arcsin(x) => recursive_apply!(Arcsin(x), name, value.clone()),
            Arccos(x) => recursive_apply!(Arccos(x), name, value.clone()),
            Arctan(x) => recursive_apply!(Arctan(x), name, value.clone()),
        };
        // If you need to modify this code, God help you.
        match partial.clone() {
            Add(a, b) => simplify_nn!(a, b => a + b, else partial),
            Mul(a, b) => simplify_nn!(a, b => a * b, else partial),
            Pow(a, b) => simplify_nn!(a, b => a.powf(b), else partial),
            Log(a, b) => simplify_nn!(a, b => a.log(b), else partial),
            Neg(x) => simplify_nn!(x => -x, else partial),
            Sin(x) => simplify_nn!(x => x.sin(), else partial),
            Cos(x) => simplify_nn!(x => x.cos(), else partial),
            Arcsin(x) => simplify_nn!(x => x.asin(), else partial),
            Arccos(x) => simplify_nn!(x => x.acos(), else partial),
            Arctan(x) => simplify_nn!(x => x.atan(), else partial),
            _ => partial
        }
    }
}

#[test]
fn one() {
    let a = 3f64;
    let b = -2f64;
    let c = 1f64;
    let quad = a*(s!(x)^2.) + b*s!(x) + c;
    quad.apply1("x", 3.).val().unwrap();
}

#[test]
fn batch() {
    let scalar_field = s!(x) * s!(y) * s!(z);
    assert_eq!(apply!(scalar_field @ x = 2., y = 3., z = 4.).val().unwrap(), 24.);
}

#[test]
fn expr() {
    let field = (s!(x)^2.) + s!(y).sqrt();
    let x_t = 2.*s!(t) + 3.;
    let y_t = s!(t) - s!(t)^2.;
    let field_t = apply!(field @ x = x_t, y = y_t);
    assert_eq!(format!("{:?}", field_t),
    "Add(Pow(Add(Mul(Num(2.0), Symbol(\"t\")), Num(3.0)), Num(2.0)), Pow(Pow(Add(Symbol(\"t\"), Neg(Symbol(\"t\"))), Num(2.0)), Num(0.5)))");
    assert_eq!(apply!(field_t @ t=1.).val().unwrap(), 25.);
}
