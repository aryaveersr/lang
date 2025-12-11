use crate::{
    mir::Value,
    ops::{BinOp, UnOp},
};

impl UnOp {
    pub fn fold(&self, value: Value) -> Value {
        match self {
            Self::Negate => Value::Num(-value.as_num()),
            Self::Not => Value::Bool(!value.as_bool()),
        }
    }
}

impl BinOp {
    pub fn fold(&self, lhs: Value, rhs: Value) -> Value {
        match self {
            Self::Add => Value::Num(lhs.as_num() + rhs.as_num()),
            Self::Sub => Value::Num(lhs.as_num() - rhs.as_num()),
            Self::Mul => Value::Num(lhs.as_num() * rhs.as_num()),
            Self::Div => Value::Num(lhs.as_num() / rhs.as_num()),
            Self::Eq => Value::Bool(lhs == rhs),
            Self::NotEq => Value::Bool(lhs != rhs),
            Self::Lesser => Value::Bool(lhs.as_num() < rhs.as_num()),
            Self::LesserEq => Value::Bool(lhs.as_num() <= rhs.as_num()),
            Self::Greater => Value::Bool(lhs.as_num() > rhs.as_num()),
            Self::GreaterEq => Value::Bool(lhs.as_num() >= rhs.as_num()),
            Self::And => Value::Bool(lhs.as_bool() && rhs.as_bool()),
            Self::Or => Value::Bool(lhs.as_bool() || rhs.as_bool()),
        }
    }
}
