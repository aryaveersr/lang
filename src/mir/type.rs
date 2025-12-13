use crate::mir::{MirType, Operand};

impl MirType {
    pub fn default_value(&self) -> Operand {
        match self {
            Self::Bool => Operand::Bool(false),
            Self::Num => Operand::Num(0),
        }
    }
}
