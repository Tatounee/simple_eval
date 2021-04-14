
use std::str::FromStr;

use crate::eval::Eval;
use crate::maph_error::Error;
use crate::parse::Parse;

use super::tree::TreeNode;
use super::function::Function;
use super::token::Tokenize;
use super::consts::Const;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Node(Box<TreeNode>),
    Number(f64, usize),
    Const(Const),
    Function(Box<Function>),
}

impl FromStr for Expr {
    type Err = Vec<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let calulation = s.tokenize()?;
        let parsable = calulation.pre_parse()?;
        Ok(parsable.parse()?)
    }
}

impl Eval for Expr {
    type Output = (f64, usize);
    type Err = Error;

    fn eval(&self) -> Result<Self::Output, Self::Err> {
        match self {
            Self::Node(node) => node.eval(),
            Self::Number(n, span) => Ok((*n, *span)),
            Self::Const(c) => c.eval(),
            Self::Function(fnc) => fnc.eval()
        }
    }
}

