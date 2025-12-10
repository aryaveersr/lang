use crate::mir::{Term, Value};

impl Term {
    pub fn operand(&mut self) -> Option<&mut Value> {
        match self {
            Self::Jump { .. } => None,
            Self::Branch { cond, .. } => Some(cond),
            Self::Return { value } => value.as_mut(),
        }
    }
}
