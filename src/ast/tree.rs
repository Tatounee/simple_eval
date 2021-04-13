
use crate::eval::Eval;
use super::expr::Expr;
use super::token::operator::Operator;

#[derive(Debug, PartialEq)]
pub struct TreeNode {
    pub l_expr: Expr,
    pub op: Operator,
    pub r_expr: Expr,
}

impl Eval for TreeNode {
    fn eval(&self) -> f64 {
        let l_number: f64 = self.l_expr.eval();
        let r_number: f64 = self.r_expr.eval();
        match self.op {
            Operator::Add => {
                l_number + r_number
            }
            Operator::Sub => {
                l_number - r_number
            }
            Operator::Mul => {
                l_number * r_number
            }
            Operator::Div => {
                l_number / r_number
            }
            Operator::Pow => {
                l_number.powf(r_number)
            }
            Operator::Mod => {
                l_number.rem_euclid(r_number)
            }
            Operator::FDiv => {
                (l_number / r_number).trunc()
            }
        }
    }
}