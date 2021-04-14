
use std::str::FromStr;

use crate::eval::Eval;
use crate::maph_error::Error;

use super::tree::TreeNode;
use super::function::Function;
use super::token::operator::Operator;
use super::token::Tokenize;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Node(Box<TreeNode>),
    Number(f64),
    Const(Const),
    Function(Box<Function>),
}

impl Eval for Expr {
    fn eval(&self) -> f64 {
        match self {
            Self::Node(node) => node.eval(),
            Self::Number(n) => *n,
            Self::Const(c) => c.eval(),
            Self::Function(fnc) => fnc.eval()
        }
    }
}

impl FromStr for Expr {
    type Err = Vec<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let calulation = s.tokenize()?;
        let parsable = calulation.pre_parse()?;
        Ok(parsable.parse()?)
    }
}

    type Err = Error;

    }
}

