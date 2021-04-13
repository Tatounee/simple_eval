
use crate::eval::Eval;
use super::expr::Expr;

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
