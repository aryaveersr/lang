use crate::mir::{Gen, Reg, VarID};

impl Reg {
    pub fn new_var(var_id: VarID, genn: Gen) -> Self {
        Self::Var { var_id, genn }
    }

    pub fn as_var(&self) -> Option<(VarID, Gen)> {
        match self {
            Self::Var { var_id, genn } => Some((*var_id, *genn)),
            Self::Temp(..) => None,
        }
    }

    pub fn get_var_id(&self) -> Option<VarID> {
        self.as_var().map(|(id, _)| id)
    }
}
