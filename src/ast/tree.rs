
use crate::eval::Eval;
use super::expr::Expr;
use super::token::operator::Operator;

#[derive(Debug, PartialEq)]
pub struct TreeNode {
    pub l_expr: Expr,
    pub op: Operator,
    pub r_expr: Expr,
}
