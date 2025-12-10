use crate::mir::{Reg, Term};

impl Term {
    pub fn operand(&mut self) -> Option<&mut Reg> {
        match self {
            Self::Jump { .. } => None,
            Self::Branch { cond, .. } => Some(cond),
            Self::Return { value } => value.as_mut(),
        }
    }
}
