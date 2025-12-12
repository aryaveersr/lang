use crate::mir::{Value, VarID};

pub enum Operand {
    Value(Value),
    Variable(VarID),
}

impl From<Value> for Operand {
    fn from(value: Value) -> Self {
        Operand::Value(value)
    }
}

impl From<VarID> for Operand {
    fn from(value: VarID) -> Self {
        Operand::Variable(value)
    }
}
