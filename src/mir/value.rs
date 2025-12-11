use crate::mir::{Reg, Value};

impl Value {
    pub fn as_num(&self) -> i32 {
        match self {
            Self::Num(value) => *value,
            Self::Bool(..) | Self::Reg(..) => unreachable!(),
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Self::Bool(value) => *value,
            Self::Num(..) | Self::Reg(..) => unreachable!(),
        }
    }

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

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::Num(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<Reg> for Value {
    fn from(reg: Reg) -> Self {
        Self::Reg(reg)
    }
}
