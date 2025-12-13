use crate::{
    mir::{InstrKind, Operand},
    ops::{BinOp, UnOp},
};

impl InstrKind {
    pub fn try_fold(&self) -> Option<Operand> {
        match self {
            Self::Call { .. } => None,

            Self::Unary { op, arg } => arg.is_const().then(|| match op {
                UnOp::Negate => Operand::Num(-arg.as_num()),
                UnOp::Not => Operand::Bool(!arg.as_bool()),
            }),

            Self::Binary { op, lhs, rhs } => (lhs.is_const() && rhs.is_const()).then(|| match op {
                BinOp::Add => Operand::Num(lhs.as_num() + rhs.as_num()),
                BinOp::Sub => Operand::Num(lhs.as_num() - rhs.as_num()),
                BinOp::Mul => Operand::Num(lhs.as_num() * rhs.as_num()),
                BinOp::Div => Operand::Num(lhs.as_num() / rhs.as_num()),

                BinOp::And => Operand::Bool(lhs.as_bool() && rhs.as_bool()),
                BinOp::Or => Operand::Bool(lhs.as_bool() || rhs.as_bool()),

                BinOp::Eq => Operand::Bool(lhs == rhs),
                BinOp::NotEq => Operand::Bool(lhs != rhs),
                BinOp::Lesser => Operand::Bool(lhs.as_num() < rhs.as_num()),
                BinOp::LesserEq => Operand::Bool(lhs.as_num() <= rhs.as_num()),
                BinOp::Greater => Operand::Bool(lhs.as_num() > rhs.as_num()),
                BinOp::GreaterEq => Operand::Bool(lhs.as_num() >= rhs.as_num()),
            }),
        }
    }
}
