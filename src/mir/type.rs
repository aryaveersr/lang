use crate::mir::{MirType, Value};

impl MirType {
    pub fn default_value(&self) -> Value {
        match self {
            Self::Bool => Value::Bool(false),
            Self::Num => Value::Num(0),
        }
    }
}
