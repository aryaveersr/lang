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
            Self::Negate => write!(f, "neg"),
            Self::Not => write!(f, "not"),
        }
    }
}

impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add => write!(f, "add"),
            Self::Sub => write!(f, "sub"),
            Self::Mul => write!(f, "mul"),
            Self::Div => write!(f, "div"),
            Self::Eq => write!(f, "eq"),
            Self::NotEq => write!(f, "neq"),
            Self::Lesser => write!(f, "lt"),
            Self::LesserEq => write!(f, "lte"),
            Self::Greater => write!(f, "gt"),
            Self::GreaterEq => write!(f, "gte"),
            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
        }
    }
}
