use crate::tree::TreeNode;

use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Node(Box<TreeNode>),
    Number(f64),
    Const(Const),
    Function(Box<Function>),
}

impl FromStr for Expr {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}


#[derive(Debug, PartialEq)]
pub enum Function {
    Abs(Expr),

}
#[derive(Debug, PartialEq)]
pub enum Const {
    Pi,
    E,
    Inf
}

#[derive(Debug)]
pub enum Error {

}