use crate::mir::{Gen, Reg, VarID};

impl Reg {
    pub fn is_var(&self) -> bool {
        match self {
            Self::Var(..) => true,
            Self::Temp(..) => false,
        }
    }

    pub fn as_var(&self) -> Option<(VarID, Gen)> {
        match self {
            Self::Var(var_id, genn) => Some((*var_id, *genn)),
            Self::Temp(..) => None,
        }
    }

    pub fn get_var_id(&self) -> Option<VarID> {
        self.as_var().map(|(id, _)| id)
    }

    pub fn as_temp(&self) -> Option<usize> {
        match self {
            Self::Temp(id) => Some(*id),
            Self::Var(..) => None,
        }
    }
}
