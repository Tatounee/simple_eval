
use crate::eval::Eval;
use crate::maph_error::{
    Error,
    ErrorKinds
};
use super::expr::Expr;

pub const FUNCTION_NAME: &[&str] = &["sqrt", "cbrq", "abs", "sin", "cos", "tan", "asin", "acos", "atan", "ln", "log", "logb", "round", "floor", "ceil", "trunc", "frac", "deg", "rad"];

#[derive(Debug, PartialEq)]
pub enum Function {
    Sqrt(Expr, usize),
    Cbrq(Expr, usize),
    Abs(Expr, usize),
    Sin(Expr, usize),
    Cos(Expr, usize),
    Tan(Expr, usize),
    ArcSin(Expr, usize),
    ArcCos(Expr, usize),
    ArcTan(Expr, usize),
    Ln(Expr, usize),
    Log(Expr, usize),
    LogBased(Expr, f64, usize),
    Round(Expr, usize),
    Floor(Expr, usize),
    Ceil(Expr, usize),
    Trunc(Expr, usize),
    Frac(Expr, usize),
    Deg(Expr, usize),
    Rad(Expr, usize),
}

impl Eval for Function {
    type Output = (f64, usize);
    type Err = Error;

    fn eval(&self) -> Result<Self::Output, Self::Err> {
        match self {
            Function::Sqrt(expr, span) => {
                let number = expr.eval()?.0;
                if number < 0. {
                    return Err(Error::new(ErrorKinds::NegativeSqrt, vec![*span]));
                }
                Ok((number.sqrt(), *span))
            }
            Function::Cbrq(expr, span) => Ok((expr.eval()?.0.cbrt(), *span)),
            Function::Abs(expr, span) => Ok((expr.eval()?.0.abs(), *span)),
            Function::Sin(expr, span) => Ok((expr.eval()?.0.sin(), *span)),
            Function::Cos(expr, span) => Ok((expr.eval()?.0.cos(), *span)),
            Function::Tan(expr, span) => Ok((expr.eval()?.0.tan(), *span)),
            Function::ArcSin(expr, span) => {
                let number = expr.eval()?.0;
                if number < -1. || number > 1. {
                    return Err(Error::new(ErrorKinds::NotInRange(-1., 1.), vec![*span]));
                }
                Ok((number.asin(), *span))
            },
            Function::ArcCos(expr, span) => {
                let number = expr.eval()?.0;
                if number < -1. || number > 1. {
                    return Err(Error::new(ErrorKinds::NotInRange(-1., 1.), vec![*span]));
                }
                Ok((number.acos(), *span))
            },
            Function::ArcTan(expr, span) => Ok((expr.eval()?.0.atan(), *span)),
            Function::Ln(expr, span) => Ok((expr.eval()?.0.ln(), *span)),
            Function::Log(expr, span) => Ok((expr.eval()?.0.log10(), *span)),
            Function::LogBased(expr, base, span) => {
                let number = expr.eval()?.0;
                match base {
                    two if two - 2. <= f64::EPSILON => Ok((number.log2(), *span)),
                    ten if ten - 10. <= f64::EPSILON => Ok((number.log10(), *span)),
                    b => Ok((number.log(*b), *span)),
                }
            },
            Function::Round(expr, span) => Ok((expr.eval()?.0.round(), *span)),
            Function::Floor(expr, span) => Ok((expr.eval()?.0.floor(), *span)),
            Function::Ceil(expr, span) => Ok((expr.eval()?.0.ceil(), *span)),
            Function::Trunc(expr, span) => Ok((expr.eval()?.0.trunc(), *span)),
            Function::Frac(expr, span) => Ok((expr.eval()?.0.fract(), *span)),
            Function::Deg(expr, span) => Ok((expr.eval()?.0.to_degrees(), *span)),
            Function::Rad(expr, span) => Ok((expr.eval()?.0.to_radians(), *span)),
        }
    }
}