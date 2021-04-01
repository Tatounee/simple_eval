
use crate::operator::Operator;

use std::str::FromStr;

const OPERATOR_CHARS: &[char] = &['+', '-', '*', '/', '%', '^', '~'];
const OTHER_VALID_CHARS: &[char] = &['(', ')', '.'];

pub type Calculation = Vec<Token>;

trait Tokenize {
    fn tokenize(&self) -> Calculation;
}

impl Tokenize for &str {
    fn tokenize(&self) -> Calculation {
        let mut number_buf = String::new();
        let mut ident_buf = String::new();
        let mut calculation = vec![];
        for c in self.replace("//", "~").replace(",", ".").chars().filter(|c| OPERATOR_CHARS.contains(c) || OTHER_VALID_CHARS.contains(c) || c.is_ascii_alphanumeric()) {
            match c {
                '0'..='9' | '.' => {
                    if !ident_buf.is_empty() {
                        calculation.push(Token::from_str(&ident_buf).unwrap());
                        ident_buf.clear();
                    }
                    number_buf.push(c)
                },
                'a'..='z' | 'A'..='Z' => {
                    if !number_buf.is_empty() {
                        calculation.push(Token::from_str(&number_buf).unwrap());
                        number_buf.clear();
                    }
                    ident_buf.push(c)
                },
                c if OPERATOR_CHARS.contains(&c) || OTHER_VALID_CHARS.contains(&c) => {
                    if !ident_buf.is_empty() {
                        calculation.push(Token::from_str(&ident_buf).unwrap());
                        ident_buf.clear();
                    }
                    if !number_buf.is_empty() {
                        calculation.push(Token::from_str(&number_buf).unwrap());
                        number_buf.clear();
                    }
                    calculation.push(Token::from_str(&c.to_string()).unwrap())
                },
                _ => unreachable!()
            }
        }
        if !ident_buf.is_empty() {
            calculation.push(Token::from_str(&ident_buf).unwrap());
        }
        if !number_buf.is_empty() {
            calculation.push(Token::from_str(&number_buf).unwrap());
        }
        calculation
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Op(Operator),
    Ident(String),
    LBrack,
    RBrack,
}

impl FromStr for Token {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = s.parse::<f64>();
        match s {
            _ if parsed.is_ok() => Ok(Self::Number(parsed.unwrap())),
            i if i.chars().all(|c | c.is_alphabetic()) => Ok(Self::Ident(i.to_owned())),
            op if OPERATOR_CHARS.contains(&s.chars().next().unwrap_or('a')) => Ok(Self::Op(Operator::from_str(op).unwrap())),
            "(" => Ok(Self::LBrack),
            ")" => Ok(Self::RBrack),
            _ => Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn tokenize_simple_addition() {
        let calculation = "1 + 2".tokenize();
        assert_eq!(calculation, vec![
            Token::Number(1.),
            Token::Op(Operator::Add),
            Token::Number(2.)
        ])
    }

    #[test]
    fn tokenize_multiple_scope() {
        let calcuation = "(2 - ((0) * 2))()((".tokenize();
        assert_eq!(calcuation, vec![
            Token::LBrack,
            Token::Number(2.),
            Token::Op(Operator::Sub),
            Token::LBrack,
            Token::LBrack,
            Token::Number(0.),
            Token::RBrack,
            Token::Op(Operator::Mul),
            Token::Number(2.),
            Token::RBrack,
            Token::RBrack,
            Token::LBrack,
            Token::RBrack,
            Token::LBrack,
            Token::LBrack,
        ])
    }

    #[test]
    fn tokenize_float_number() {
        let calculation = "42.69".tokenize();
        assert_eq!(calculation, vec![
            Token::Number(42.69)
        ])
    }

    #[test]
    fn tokenize_float_number_without_decimal_part() {
        let calculation = "13105.".tokenize();
        assert_eq!(calculation, vec![
            Token::Number(13105.)
        ])
    }

    #[test]
    fn tokenize_double_slash() {
        let calculation = "2 // 3".tokenize();
        assert_eq!(calculation, vec![
            Token::Number(2.),
            Token::Op(Operator::FDiv),
            Token::Number(3.)
        ])
    }    
    
    #[test]
    fn tokenize_triple_slash() {
        let calculation = "2 /// 3".tokenize();
        assert_eq!(calculation, vec![
            Token::Number(2.),
            Token::Op(Operator::FDiv),
            Token::Op(Operator::Div),
            Token::Number(3.)
        ])
    }

    #[test]
    fn tokenize_all_operators() {
        let calculation = "/ * - + % ~ ^".tokenize();
        assert_eq!(calculation, vec![
            Token::Op(Operator::Div),
            Token::Op(Operator::Mul),
            Token::Op(Operator::Sub),
            Token::Op(Operator::Add),
            Token::Op(Operator::Mod),
            Token::Op(Operator::FDiv),
            Token::Op(Operator::Pow),
        ])
    }

    #[test]
    fn tokenize_ident_and_number() {
        let calculation = "sin45.2aBcd+PI-8".tokenize();
        assert_eq!(calculation, vec![
            Token::Ident("sin".to_owned()),
            Token::Number(45.2),
            Token::Ident("aBcd".to_owned()),
            Token::Op(Operator::Add),
            Token::Ident("PI".to_owned()),
            Token::Op(Operator::Sub),
            Token::Number(8.)
        ])
    }
}