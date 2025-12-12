use crate::mir::{Value, VarID};

#[derive(Debug, Clone, Copy)]
pub enum Operand {
    Value(Value),
    Variable(VarID),
}

impl From<Value> for Operand {
    fn from(value: Value) -> Self {
        Self::Value(value)
    }
}

impl From<VarID> for Operand {
    fn from(value: VarID) -> Self {
        Self::Variable(value)
    }
}
