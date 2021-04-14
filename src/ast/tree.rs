
use crate::eval::Eval;
use crate::maph_error::{
    Error,
    ErrorKinds
};

use super::expr::Expr;
use super::token::operator::Operator;

#[derive(Debug, PartialEq)]
pub struct TreeNode {
    pub l_expr: Expr,
    pub op: Operator,
    pub r_expr: Expr,
}

impl Eval for TreeNode {
    type Output = (f64, usize);
    type Err = Error;

    fn eval(&self) -> Result<Self::Output, Self::Err> {
        let l_number = self.l_expr.eval()?;
        let r_number = self.r_expr.eval()?;
        match self.op {
            Operator::Add => {
                Ok((l_number.0 + r_number.0, l_number.1))
            }
            Operator::Sub => {
                Ok((l_number.0 - r_number.0, l_number.1))
            }
            Operator::Mul => {
                Ok((l_number.0 * r_number.0, l_number.1))
            }
            Operator::Div => {
                if r_number.0 == 0. {
                    return Err(Error::new(ErrorKinds::DivisionByZero, vec![r_number.1]));
                }
                Ok((l_number.0 / r_number.0, l_number.1))
            }
            Operator::Pow => {
                Ok((l_number.0.powf(r_number.0), l_number.1))
            }
            Operator::Mod => {
                Ok((l_number.0.rem_euclid(r_number.0), l_number.1))
            }
            Operator::FDiv => {
                if r_number.0 == 0. {
                    return Err(Error::new(ErrorKinds::ModuloByZero, vec![r_number.1]));
                }
                Ok(((l_number.0 / r_number.0).trunc(), l_number.1))
            }
        }
    }
}