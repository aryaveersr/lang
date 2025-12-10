use crate::mir::{Gen, Register, VarID};

impl Register {
    pub fn is_var(&self) -> bool {
        match self {
            Register::Var(_, _) => true,
            _ => false,
        }
    }

    pub fn as_var(&self) -> Option<(VarID, Gen)> {
        match self {
            Register::Var(var_id, genn) => Some((*var_id, *genn)),
            _ => None,
        }
    }

    pub fn get_var_id(&self) -> Option<VarID> {
        self.as_var().map(|(id, _)| id)
    }

    pub fn as_temp(&self) -> Option<usize> {
        match self {
            Register::Temp(id) => Some(*id),
            _ => None,
        }
    }
}
