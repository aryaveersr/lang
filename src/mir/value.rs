use crate::mir::{Reg, Value};

impl Value {
    pub fn as_reg(&self) -> Option<Reg> {
        match self {
            Self::Reg(reg) => Some(*reg),
            Self::Bool(..) | Self::Num(..) => None,
        }
    }

    pub fn as_reg_mut(&mut self) -> Option<&mut Reg> {
        match self {
            Self::Reg(reg) => Some(reg),
            Self::Bool(..) | Self::Num(..) => None,
        }
    }
}

impl From<Reg> for Value {
    fn from(reg: Reg) -> Self {
        Self::Reg(reg)
    }
}
