
use std::collections::VecDeque;

use crate::ast::{
    expr::Expr,
    function::Function,
    tree::TreeNode
};

use crate::ast::token::{Calculation, TokenKind, operator::Operator};
use crate::maph_error::{Error, ErrorKinds};

trait Parsable {
    type Err;
    type Output;

    fn pre_parse(self) -> Result<Self::Output, Self::Err>;
    fn verify_scoping(&self) -> Result<(), Self::Err>;
    fn verify_operator(&self) -> Result<(), Self::Err>;
    fn concat_minus_and_number(&mut self);
}

impl Parsable for Calculation {
    type Err = Vec<Error>;
    type Output = Self;

    fn pre_parse(mut self) -> Result<Self::Output, Self::Err> {
        
        let mut errors = vec![];
        
        match self.verify_scoping() {
            Err(mut e) => errors.append(&mut e),
            Ok(_) => {}
        }

        self.concat_minus_and_number();
        match self.verify_operator() {
            Err(mut e) => errors.append(&mut e),
            Ok(_) => {}
        }
        
        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(self)
        }
    }

    fn verify_scoping(&self) -> Result<(), Self::Err> {
        let mut deep = VecDeque::new();
        for tk in self.iter() {
            match tk.token_kind {
                TokenKind::LBrack => deep.push_back(tk.span),
                TokenKind::RBrack => {
                    if let None = deep.pop_back() {
                        return Err(vec![Error::new(ErrorKinds::UnexeptedBrack, vec![tk.span])])
                    }
                },
                _ => {}
            }
        }
        if deep.len() > 0 {
            Err(vec![Error::new(ErrorKinds::UnclosedBrack, deep.drain(0..deep.len()).collect())])
        } else {
            Ok(())
        }
    }

    fn verify_operator(&self) -> Result<(), Self::Err> {
        let mut errors = vec![];
        let mut pre_operator = (false, 0);
        let mut error_raise = false;
        for tk in self.iter() {
            match tk.token_kind {
                TokenKind::Op(op) => {
                    if !error_raise && pre_operator.0 {
                        error_raise = true;
                        errors.push(Error::new(ErrorKinds::UnexeptedOperator(op.clone()), vec![pre_operator.1]))
                    }
                    if !pre_operator.0 {
                        pre_operator = (true, tk.span)
                    }
                }
                _ => {
                    pre_operator = (false, 0);
                    error_raise = false;
                },
            }
        }
        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(())
        }
    }

    fn concat_minus_and_number(&mut self) {
        let mut sub_index = vec![];
        let mut number = None;
        for (idx, tk) in self.iter_mut().enumerate().rev() {
            match tk.token_kind {
                TokenKind::Number(ref mut x) => number = Some(x),
                TokenKind::Op(Operator::Sub) => if let Some(ref mut x) = number {
                    **x = - **x;
                    sub_index.push(idx)
                },
                _ => number = None
            }
        }
        for i in sub_index.into_iter() {
            self.remove(i);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pre_parse_add() {
        let calculation = "2 + 3".tokenize().unwrap();
        let parsable = calculation.pre_parse().unwrap();
        assert_eq!(
            parsable,
            vec![
                token!(Number(2.), 0),
                token!(Op::Add, 2),
                token!(Number(3.), 4)
            ]
        );
    }
}
