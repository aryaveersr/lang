use crate::mir::{Instr, InstrKind, Value};

impl Instr {
    pub fn update_operands<F: FnMut(&mut Value)>(&mut self, mut f: F) {
        match &mut self.kind {
            InstrKind::ConstBool { .. } | InstrKind::ConstNum { .. } => {}

            InstrKind::Copy { src } | InstrKind::Unary { arg: src, .. } => {
                f(src);
            }

            InstrKind::Binary { lhs, rhs, .. } => {
                f(lhs);
                f(rhs);
            }

            InstrKind::Call { args, .. } => {
                args.iter_mut().for_each(f);
            }
        }
    }
}
