use std::fmt::{self, Display, Formatter};

use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize, Clone, Copy)]
pub enum UnOp {
    Negate,
    Not,
}

#[derive(Debug, PartialEq, Eq, Serialize, Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,

    Eq,
    NotEq,
    Lesser,
    LesserEq,
    Greater,
    GreaterEq,

    And,
    Or,
}

impl Display for UnOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Negate => write!(f, "-"),
            Self::Not => write!(f, "!"),
        }
    }
}

impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Eq => write!(f, "=="),
            Self::NotEq => write!(f, "!="),
            Self::Lesser => write!(f, "<"),
            Self::LesserEq => write!(f, "<="),
            Self::Greater => write!(f, ">"),
            Self::GreaterEq => write!(f, ">="),
            Self::And => write!(f, "&&"),
            Self::Or => write!(f, "||"),
        }
    }
}
