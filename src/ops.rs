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
