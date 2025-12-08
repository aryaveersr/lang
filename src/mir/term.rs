use crate::mir::{Term, ValueID};

impl Term {
    pub fn operands(&mut self) -> Option<&mut ValueID> {
        match self {
            Self::Jump { .. } => None,
            Self::Branch { cond, .. } => Some(cond),
            Self::Return { value } => value.as_mut(),
        }
    }
}
