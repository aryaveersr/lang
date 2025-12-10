use crate::mir::{Reg, Value};

impl From<Reg> for Value {
    fn from(reg: Reg) -> Self {
        Value::Reg(reg)
    }
}
