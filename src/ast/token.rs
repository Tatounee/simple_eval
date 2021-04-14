
use std::{str::FromStr, usize};

use lazy_static::lazy_static;
use regex::Regex;

pub mod operator;

use operator::Operator;

use crate::maph_error::{Error, ErrorKinds};
use crate::utils::DedupReplaceFor;

const OPERATOR_CHARS: &[char] = &['+', '-', '*', '/', '%', '^', '~'];
const OTHER_VALID_CHARS: &[char] = &['(', ')', '.', ',', ' '];

pub type Calculation = Vec<Token>;

pub trait Tokenize {
    type Err;
    fn tokenize(&self) -> Result<Calculation, Self::Err>;
}

impl Tokenize for &str {
    type Err = Vec<Error>;

    fn tokenize(&self) -> Result<Calculation, Self::Err> {
        let input_len = self.len();

        let mut input_vec = self
            .trim_end()
            .replace(",", ".")
            .chars()
            .enumerate()
            .skip_while(|(_, c)| c.is_whitespace())
            .collect::<Vec<_>>();
        input_vec.dedup_by(|(_, a), (_, b)| a == b && a == &' ');
        input_vec.dedup_and_replace_for_by_key_then_build(&'/', 2, |(_, c)| c, |(i, _)|  (i - 1, '~'));

        let mut errors = input_vec
            .iter()
            .filter_map(|(i, c)| {
                if !(OPERATOR_CHARS.contains(c)
                    || OTHER_VALID_CHARS.contains(c)
                    || c.is_ascii_alphanumeric())
                {
                    Some(Error::new(ErrorKinds::InvalideChar(*c), vec![*i]))
                } else {
                    None
                }
            })
            .collect::<Vec<Error>>();

        let mut buffer = String::new();
        let mut buffer_type = BufferType::None;
        let mut calculation = vec![];

        for (index, c) in input_vec.into_iter().filter(|(_, c)| {
            OPERATOR_CHARS.contains(c) || OTHER_VALID_CHARS.contains(c) || c.is_ascii_alphanumeric()
        }) {
            match c {
                '0'..='9' | '.' => {
                    match buffer_type {
                        BufferType::None => {
                            buffer_type = BufferType::Number;
                        }
                        _ => {}
                    }
                    buffer.push(c);
                }
                'a'..='z' | 'A'..='Z' => {
                    match buffer_type {
                        BufferType::Number => {
                            match TokenKind::from_str(&buffer) {
                                Ok(tk) => {
                                    calculation.push(Token::new(tk, index - buffer.len()));
                                    buffer.clear();
                                }
                                Err(mut e) => {
                                    e.add_span(index - 1);
                                    errors.push(e);
                                    buffer.clear();
                                }
                            }
                            buffer_type = BufferType::Ident;
                        }
                        BufferType::None => {
                            buffer_type = BufferType::Ident;
                        }
                        _ => {}
                    }
                    buffer.push(c);
                }
                c if OPERATOR_CHARS.contains(&c) || OTHER_VALID_CHARS.contains(&c) => {
                    match buffer_type {
                        BufferType::Ident | BufferType::Number => {
                            match TokenKind::from_str(&buffer) {
                                Ok(tk) => {
                                    calculation.push(Token::new(tk, index - buffer.len()));
                                    buffer.clear();
                                }
                                Err(mut e) => {
                                    e.add_span(index - 1);
                                    errors.push(e);
                                    buffer.clear();
                                }
                            }
                            buffer_type = BufferType::None;
                        }
                        _ => {}
                    }
                    if c == ' ' {
                        continue;
                    }
                    match TokenKind::from_str(&c.to_string()) {
                        Ok(tk) => calculation.push(Token::new(tk, index)),
                        Err(e) => errors.push(e),
                    }
                }
                _ => unreachable!(),
            }
        }

        if !matches!(buffer_type, BufferType::None) {
            match TokenKind::from_str(&buffer) {
                Ok(tk) => {
                    calculation.push(Token::new(tk, input_len - buffer.len()));
                }
                Err(mut e) => {
                    e.add_span(input_len - 1);
                    errors.push(e);
                }
            }
        }

        if errors.is_empty() {
            Ok(calculation)
        } else {
            Err(errors)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_kind: TokenKind,
    pub span: usize,
}

impl Token {
    pub fn new(token_kind: TokenKind, span: usize) -> Self {
        Self { token_kind, span }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Number(f64),
    Op(Operator),
    Ident(String),
    LBrack,
    RBrack,
}

impl FromStr for TokenKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE_IDENT: Regex = Regex::new(r#"^((?i)[a-z])((?i)[a-z0-9])*$"#).unwrap();
        };
        let commas = s
            .chars()
            .rev()
            .enumerate()
            .filter_map(|(i, c)| if c == '.' { Some(i) } else { None })
            .collect::<Vec<usize>>();
        if commas.len() > 1 {
            return Err(Error::new(ErrorKinds::MultipleComma, commas));
        }

        let parsed = s.parse::<f64>();
        match s {
            _ if parsed.is_ok() => Ok(Self::Number(parsed.unwrap())),
            i if RE_IDENT.is_match(s) => Ok(Self::Ident(i.to_owned())),
            op if OPERATOR_CHARS.contains(&s.chars().next().unwrap_or_default()) => {
                Ok(Self::Op(Operator::from_str(op).unwrap()))
            }
            "(" => Ok(Self::LBrack),
            ")" => Ok(Self::RBrack),
            other => Err(Error::new(
                ErrorKinds::UnknowOperator(other.to_owned()),
                vec![],
            )),
        }
    }
}

enum BufferType {
    Ident,
    Number,
    None,
}

#[macro_export]
macro_rules! token {
    ($tk_kind:ident $( ( $($enum_data:expr),* ) )?, $span:expr) => {
        crate::ast::token::Token {
            token_kind: crate::ast::token::TokenKind::$tk_kind$( ( $($enum_data)* ) )?,
            span: $span
        }
    };
    (Op::$op_kind:ident, $span:expr) => {
        crate::ast::token::Token {
            token_kind: crate::ast::token::TokenKind::Op(Operator::$op_kind),
            span: $span
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn tokenize_unknow_char() {
        let calculation = "$".tokenize();
        assert_eq!(
            calculation,
            Err(vec![Error::new(ErrorKinds::InvalideChar('$'), vec![0])])
        )
    }

    #[test]
    fn tokenize_multiple_unknow_chars() {
        let calculation = "$¨\" 123".tokenize();
        assert_eq!(
            calculation,
            Err(vec![
                Error::new(ErrorKinds::InvalideChar('$'), vec![0]),
                Error::new(ErrorKinds::InvalideChar('¨'), vec![1]),
                Error::new(ErrorKinds::InvalideChar('"'), vec![2]),
            ])
        )
    }

    #[test]
    fn tokenize_simple_addition() {
        let calculation = "1 + 2".tokenize();
        assert_eq!(
            calculation,
            Ok(vec![
                token!(Number(1.), 0),
                token!(Op::Add, 2),
                token!(Number(2.), 4),
            ])
        );
    }

    #[test]
    fn tokenize_multiple_scope() {
        let calcuation = "(2 - ((0) * 2))()((".tokenize();
        assert_eq!(
            calcuation,
            Ok(vec![
                token!(LBrack, 0),
                token!(Number(2.), 1),
                token!(Op::Sub, 3),
                token!(LBrack, 5),
                token!(LBrack, 6),
                token!(Number(0.), 7),
                token!(RBrack, 8),
                token!(Op::Mul, 10),
                token!(Number(2.), 12),
                token!(RBrack, 13),
                token!(RBrack, 14),
                token!(LBrack, 15),
                token!(RBrack, 16),
                token!(LBrack, 17),
                token!(LBrack, 18),
            ])
        )
    }

    #[test]
    fn tokenize_float_number() {
        let calculation = "42.69".tokenize();
        assert_eq!(calculation, Ok(vec![token!(Number(42.69), 0)]))
    }

    #[test]
    fn tokenize_float_number_without_decimal_part() {
        let calculation = "13105.".tokenize();
        assert_eq!(calculation, Ok(vec![token!(Number(13105.), 0)]))
    }

    #[test]
    fn tokenize_float_number_with_multiple_comma() {
        let calculation = "131.05.".tokenize();
        assert_eq!(
            calculation,
            Err(vec![Error::new(ErrorKinds::MultipleComma, vec![6, 3])])
        )
    }

    #[test]
    fn tokenize_shift_left_float_number_with_multiple_comma() {
        let calculation = "150 131.05.".tokenize();
        assert_eq!(
            calculation,
            Err(vec![Error::new(ErrorKinds::MultipleComma, vec![10, 7])])
        )
    }

    #[test]
    fn tokenize_shift_right_float_number_with_multiple_comma() {
        let calculation = "131.05. 150".tokenize();
        assert_eq!(
            calculation,
            Err(vec![Error::new(ErrorKinds::MultipleComma, vec![6, 3])])
        )
    }

    #[test]
    fn tokenize_multiple_float_number_with_multiple_comma() {
        let calculation = "151.64. + 99.66.33.11".tokenize();
        assert_eq!(
            calculation,
            Err(vec![
                Error::new(ErrorKinds::MultipleComma, vec![6, 3]),
                Error::new(ErrorKinds::MultipleComma, vec![18, 15, 12]),
            ])
        )
    }

    #[test]
    fn tokenize_multiple_number() {
        let caluctation = "177 013".tokenize();
        assert_eq!(
            caluctation,
            Ok(vec![token!(Number(177.), 0), token!(Number(13.), 4)])
        );
    }

    #[test]
    fn tokenize_double_slash() {
        let calculation = "2 // 3".tokenize();
        assert_eq!(
            calculation,
            Ok(vec![
                token!(Number(2.), 0),
                token!(Op::FDiv, 2),
                token!(Number(3.), 5),
            ])
        )
    }

    #[test]
    fn tokenize_triple_slash() {
        let calculation = "2 /// 3".tokenize();
        assert_eq!(
            calculation,
            Ok(vec![
                token!(Number(2.), 0),
                token!(Op::FDiv, 2),
                token!(Op::Div, 4),
                token!(Number(3.), 6),
            ])
        )
    }

    #[test]
    fn tokenize_all_operators() {
        let calculation = "/ * - + % ~ ^ //".tokenize();
        assert_eq!(
            calculation,
            Ok(vec![
                token!(Op::Div, 0),
                token!(Op::Mul, 2),
                token!(Op::Sub, 4),
                token!(Op::Add, 6),
                token!(Op::Mod, 8),
                token!(Op::FDiv, 10),
                token!(Op::Pow, 12),
                token!(Op::FDiv, 14),
            ])
        )
    }

    #[test]
    fn tokenize_ident_and_number() {
        let calculation = "sin452~2aB cd+PI-8".tokenize();
        assert_eq!(
            calculation,
            Ok(vec![
                token!(Ident("sin452".to_owned()), 0),
                token!(Op::FDiv, 6),
                token!(Number(2.), 7),
                token!(Ident("aB".to_owned()), 8),
                token!(Ident("cd".to_owned()), 11),
                token!(Op::Add, 13),
                token!(Ident("PI".to_owned()), 14),
                token!(Op::Sub, 16),
                token!(Number(8.), 17),
            ])
        )
    }

    #[test]
    fn tokenize_complex_calculation() {
        let calculation = "sin(5E-2) - 1/2(366^3) // 31 ~ 5".tokenize();
        assert_eq!(calculation, Ok(vec![
            token!(Ident("sin".to_owned()), 0),
            token!(LBrack, 3),
            token!(Number(5.), 4),
            token!(Ident("E".to_owned()), 5),
            token!(Op::Sub, 6),
            token!(Number(2.), 7),
            token!(RBrack, 8),
            token!(Op::Sub, 10),
            token!(Number(1.), 12),
            token!(Op::Div, 13),
            token!(Number(2.), 14),
            token!(LBrack, 15),
            token!(Number(366.), 16),
            token!(Op::Pow, 19),
            token!(Number(3.), 20),
            token!(RBrack, 21),
            token!(Op::FDiv, 23),
            token!(Number(31.), 26),
            token!(Op::FDiv, 29),
            token!(Number(5.), 31),
        ]));
    }
}
