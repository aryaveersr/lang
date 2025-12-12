use crate::{
    mir::{InstrKind, Value},
    ops::{BinOp, UnOp},
};

impl InstrKind {
    pub fn try_fold(&self) -> Option<Value> {
        match self {
            InstrKind::Call { .. } => None,

            InstrKind::Unary { op, arg } => arg.is_const().then(|| match op {
                UnOp::Negate => Value::Num(-arg.as_num()),
                UnOp::Not => Value::Bool(!arg.as_bool()),
            }),

            InstrKind::Binary { op, lhs, rhs } => {
                (lhs.is_const() && rhs.is_const()).then(|| match op {
                    BinOp::Add => Value::Num(lhs.as_num() + rhs.as_num()),
                    BinOp::Sub => Value::Num(lhs.as_num() - rhs.as_num()),
                    BinOp::Mul => Value::Num(lhs.as_num() * rhs.as_num()),
                    BinOp::Div => Value::Num(lhs.as_num() / rhs.as_num()),

                    BinOp::And => Value::Bool(lhs.as_bool() && rhs.as_bool()),
                    BinOp::Or => Value::Bool(lhs.as_bool() || rhs.as_bool()),

                    BinOp::Eq => Value::Bool(lhs == rhs),
                    BinOp::NotEq => Value::Bool(lhs != rhs),
                    BinOp::Lesser => Value::Bool(lhs.as_num() < rhs.as_num()),
                    BinOp::LesserEq => Value::Bool(lhs.as_num() <= rhs.as_num()),
                    BinOp::Greater => Value::Bool(lhs.as_num() > rhs.as_num()),
                    BinOp::GreaterEq => Value::Bool(lhs.as_num() >= rhs.as_num()),
                })
            }
        }
    }
}
