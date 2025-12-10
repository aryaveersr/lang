use crate::mir::{Reg, Value};

impl Value {
    pub fn as_reg(&self) -> Option<Reg> {
        match self {
            Value::Reg(reg) => Some(*reg),
            Value::Bool(..) | Value::Num(..) => None,
        }
    }

    pub fn as_reg_mut(&mut self) -> Option<&mut Reg> {
        match self {
            Value::Reg(reg) => Some(reg),
            Value::Bool(..) | Value::Num(..) => None,
        }
    }
}

impl From<Reg> for Value {
    fn from(reg: Reg) -> Self {
        Value::Reg(reg)
    }
}
