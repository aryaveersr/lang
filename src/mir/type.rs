use crate::mir::{MirType, Value};

impl MirType {
    pub fn default_value(&self) -> Value {
        match self {
            Self::Bool => false.into(),
            Self::Num => 0.into(),
        }
    }
}
