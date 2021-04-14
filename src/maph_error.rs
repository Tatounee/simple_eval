use crate::ast::token::operator::Operator;

#[derive(Debug, PartialEq)]
pub struct Error {
    pub kind: ErrorKinds,
    pub span: Vec<usize>,
}

impl Error {
    pub fn new(kind: ErrorKinds, span: Vec<usize>) -> Self {
        Self { kind, span }
    }

    #[inline]
    pub fn add_span(&mut self, span: usize) {
        for s in self.span.iter_mut() {
            *s = span - *s
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorKinds {
    InvalideChar(char),
    MultipleComma,
    UnknowOperator(String),
    UnclosedBrack,
    UnexeptedBrack,
    UnexeptedOperator(Operator),
    DivisionByZero,
    ModuloByZero,
    NegativeSqrt,
    NotInRange(f64, f64),
    UnknowFonction(String)
}
