
#![allow(dead_code)]
#![allow(unused_imports)]

mod expr;
mod tree;
mod operator;
mod token;

use expr::Expr;
use tree::TreeNode;
use operator::Operator;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn parse_add() {
        let expr: Expr = "2 + 3".parse().unwrap();
        assert_eq!(expr, Expr::Node(
            Box::new(TreeNode{
                l_expr: Expr::Number(2.),
                op: Operator::Add,
                r_expr: Expr::Number(3.)
            })
        ))
    }
}