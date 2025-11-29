use serde::Serialize;
use std::fmt::{self, Display, Formatter};

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
            UnOp::Negate => write!(f, "-"),
            UnOp::Not => write!(f, "!"),
        }
    }
}

impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BinOp::Add => write!(f, "+"),
            BinOp::Sub => write!(f, "-"),
            BinOp::Mul => write!(f, "*"),
            BinOp::Div => write!(f, "/"),
            BinOp::Eq => write!(f, "=="),
            BinOp::NotEq => write!(f, "!="),
            BinOp::Lesser => write!(f, "<"),
            BinOp::LesserEq => write!(f, "<="),
            BinOp::Greater => write!(f, ">"),
            BinOp::GreaterEq => write!(f, ">="),
            BinOp::And => write!(f, "&&"),
            BinOp::Or => write!(f, "||"),
        }
    }
}
