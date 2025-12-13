use crate::{
    mir::{Operand, Reg},
    mir_builder::VarID,
};

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Operand(Operand),
    Variable(VarID),
}

impl Value {
    pub fn bool(value: bool) -> Self {
        Self::Operand(Operand::Bool(value))
    }

    pub fn num(value: i32) -> Self {
        Self::Operand(Operand::Num(value))
    }

    pub fn reg(value: Reg) -> Self {
        Self::Operand(Operand::Reg(value))
    }
}

impl From<Operand> for Value {
    fn from(value: Operand) -> Self {
        Self::Operand(value)
    }
}

impl From<VarID> for Value {
    fn from(value: VarID) -> Self {
        Self::Variable(value)
    }
}
