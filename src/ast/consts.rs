
use crate::eval::Eval;
use crate::maph_error::Error;

#[derive(Debug, PartialEq)]
pub enum Const {
    Pi(usize),
    E(usize),
    Inf(usize),
}

impl Eval for Const {
    type Output = (f64, usize);
    type Err = Error;

    fn eval(&self) -> Result<Self::Output, Self::Err> {
        match self {
            Self::Pi(span) => Ok((std::f64::consts::PI, *span)),
            Self::E(span) => Ok((std::f64::consts::E, *span)),
            Self::Inf(span) => Ok((f64::INFINITY, *span))
        }
    }
}