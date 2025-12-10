use crate::mir::{Term, Value};

impl Term {
    pub fn update_operand<F: FnMut(&mut Value)>(&mut self, mut f: F) {
        match self {
            Self::Jump { .. } | Self::Return { value: None } => {}
            Self::Return { value: Some(cond) } | Self::Branch { cond, .. } => f(cond),
        }
    }
}
