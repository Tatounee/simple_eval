
use crate::eval::Eval;
use super::expr::Expr;

pub const FUNCTION_NAME: &[&str] = &["sqrt", "cbrq", "abs", "sin", "cos", "tan", "asin", "acos", "atan", "ln", "log", "logb", "round", "floor", "ceil", "trunc", "frac", "deg", "rad"];

#[derive(Debug, PartialEq)]
pub enum Function {
    Sqrt(Expr),
    Cbrq(Expr),
    Abs(Expr),
    Sin(Expr),
    Cos(Expr),
    Tan(Expr),
    ArcSin(Expr),
    ArcCos(Expr),
    ArcTan(Expr),
    Ln(Expr),
    Log(Expr),
    LogBased(Expr, f64),
    Round(Expr),
    Floor(Expr),
    Ceil(Expr),
    Trunc(Expr),
    Frac(Expr),
    Deg(Expr),
    Rad(Expr),
}

impl Eval for Function {
    fn eval(&self) -> f64 {
        match self {
            Function::Sqrt(expr) => expr.eval().sqrt(),
            Function::Cbrq(expr) => expr.eval().cbrt(),
            Function::Abs(expr) => expr.eval().abs(),
            Function::Sin(expr) => expr.eval().sin(),
            Function::Cos(expr) => expr.eval().cos(),
            Function::Tan(expr) => expr.eval().tan(),
            Function::ArcSin(expr) => expr.eval().asin(),
            Function::ArcCos(expr) => expr.eval().acos(),
            Function::ArcTan(expr) => expr.eval().atan(),
            Function::Ln(expr) => expr.eval().ln(),
            Function::Log(expr) => expr.eval().log10(),
            Function::LogBased(expr, base) => expr.eval().log(*base),
            Function::Round(expr) => expr.eval().round(),
            Function::Floor(expr) => expr.eval().floor(),
            Function::Ceil(expr) => expr.eval().ceil(),
            Function::Trunc(expr) => expr.eval().trunc(),
            Function::Frac(expr) => expr.eval().fract(),
            Function::Deg(expr) => expr.eval().to_degrees(),
            Function::Rad(expr) => expr.eval().to_radians(),
        }
    }
}