use crate::mir::{Generation, Register, Variable};

impl Register {
    pub fn is_variable(&self) -> bool {
        match self {
            Register::Variable(_, _) => true,
            _ => false,
        }
    }

    pub fn as_variable(&self) -> Option<(Variable, Generation)> {
        match self {
            Register::Variable(variable, generation) => Some((*variable, *generation)),
            _ => None,
        }
    }

    pub fn as_temporary(&self) -> Option<usize> {
        match self {
            Register::Temporary(id) => Some(*id),
            _ => None,
        }
    }
}
