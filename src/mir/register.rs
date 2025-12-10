use crate::mir::{Generation, Register, VariableID};

impl Register {
    pub fn is_variable(&self) -> bool {
        match self {
            Register::Variable(_, _) => true,
            _ => false,
        }
    }

    pub fn as_variable(&self) -> Option<(VariableID, Generation)> {
        match self {
            Register::Variable(variable_id, generation) => Some((*variable_id, *generation)),
            _ => None,
        }
    }

    pub fn get_variable_id(&self) -> Option<VariableID> {
        self.as_variable().map(|(id, _)| id)
    }

    pub fn as_temporary(&self) -> Option<usize> {
        match self {
            Register::Temporary(id) => Some(*id),
            _ => None,
        }
    }
}
