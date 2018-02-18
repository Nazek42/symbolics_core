#[macro_use]
extern crate symbolics_core;
use symbolics_core::consts::e;

#[test]
fn quadratic_s() {
    let quad = s!(a) * (s!(x) ^ 2.) + s!(b) * s!(x) + s!(c);
    assert_eq!(format!("{:?}", quad),
    "Add(Add(Mul(Symbol(\"a\"), Pow(Symbol(\"x\"), Num(2.0))), Mul(Symbol(\"b\"), Symbol(\"x\"))), Symbol(\"c\"))");;
}

#[test]
fn exponential_nref() {
    let k = s!(k);
    let t = s!(t);
    let exp = e() ^ (-!&k * !&t);
    assert_eq!(format!("{:?}", exp),
    "Pow(Symbol(\"$e\"), Mul(Mul(Symbol(\"k\"), Num(-1.0)), Symbol(\"t\")))");
}
