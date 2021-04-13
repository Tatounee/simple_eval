
use std::str;

use crate::eval::Eval;

use super::tree::TreeNode;
use super::function::Function;
use super::token::operator::Operator;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Node(Box<TreeNode>),
    Number(f64),
    Const(Const),
    Function(Box<Function>),
}
#[derive(Debug, PartialEq)]
pub enum Const {
    Pi,
    E,
    Inf,
}
impl str::FromStr for Expr {
    type Err = Error;

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug)]
pub enum Error {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn parse_add() {
        let expr: Expr = "2 + 3".parse().unwrap();
        assert_eq!(
            expr,
            Expr::Node(Box::new(TreeNode {
                l_expr: Expr::Number(2.),
                op: Operator::Add,
                r_expr: Expr::Number(3.)
            }))
        )
    }
}
