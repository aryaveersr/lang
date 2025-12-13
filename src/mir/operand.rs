use crate::mir::Operand;

impl Operand {
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

    pub fn is_const(&self) -> bool {
        match self {
            Self::Num(..) | Self::Bool(..) => true,
            Self::Reg(..) => false,
        }
    }
}
