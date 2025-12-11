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

    pub fn as_reg(&self) -> Reg {
        match self {
            Self::Reg(reg) => *reg,
            Self::Bool(..) | Self::Num(..) => unreachable!(),
        }
    }
}
