use crate::{
    mir::Value,
    ops::{BinOp, UnOp},
};

impl UnOp {
    pub fn fold(&self, value: Value) -> Value {
        match self {
            UnOp::Negate => Value::Num(-value.as_num()),
            UnOp::Not => Value::Bool(!value.as_bool()),
        }
    }
}

impl BinOp {
    pub fn fold(&self, lhs: Value, rhs: Value) -> Value {
        match self {
            BinOp::Add => Value::Num(lhs.as_num() + rhs.as_num()),
            BinOp::Sub => Value::Num(lhs.as_num() - rhs.as_num()),
            BinOp::Mul => Value::Num(lhs.as_num() * rhs.as_num()),
            BinOp::Div => Value::Num(lhs.as_num() / rhs.as_num()),
            BinOp::Eq => Value::Bool(lhs == rhs),
            BinOp::NotEq => Value::Bool(lhs != rhs),
            BinOp::Lesser => Value::Bool(lhs.as_num() < rhs.as_num()),
            BinOp::LesserEq => Value::Bool(lhs.as_num() <= rhs.as_num()),
            BinOp::Greater => Value::Bool(lhs.as_num() > rhs.as_num()),
            BinOp::GreaterEq => Value::Bool(lhs.as_num() >= rhs.as_num()),
            BinOp::And => Value::Bool(lhs.as_bool() && rhs.as_bool()),
            BinOp::Or => Value::Bool(lhs.as_bool() || rhs.as_bool()),
        }
    }
}
