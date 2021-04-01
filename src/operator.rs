
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add, //  +
    Sub, //  -
    Mul, //  *
    Div, //  /
    Pow, //  ^
    Mod, //  %
    FDiv, //  //
}

impl FromStr for Operator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Sub),
            "*" => Ok(Self::Mul),
            "/" => Ok(Self::Div),
            "^" => Ok(Self::Pow),
            "%" => Ok(Self::Mod),
            "//" | "~" => Ok(Self::FDiv),
            _ => Err(format!("Try to create an Operator from an unkwon &str : `{}`", s))
        }
    }
}