use crate::expr::Expr;
use crate::operator::Operator;

#[derive(Debug, PartialEq)]
pub struct TreeNode {
    pub l_expr: Expr,
    pub op: Operator,
    pub r_expr: Expr
}