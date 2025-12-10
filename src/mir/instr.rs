use crate::mir::{Instr, InstrKind, Register};

impl Instr {
    pub fn operands(&mut self) -> OperandIter<'_> {
        let mut operands = Vec::new();

        match &mut self.kind {
            InstrKind::ConstBool { .. } | InstrKind::ConstNum { .. } => {}

            InstrKind::Copy { src } | InstrKind::Unary { arg: src, .. } => {
                operands.push(src);
            }

            InstrKind::Binary { lhs, rhs, .. } => {
                operands.push(lhs);
                operands.push(rhs);
            }

            InstrKind::Call { args, .. } => {
                operands.extend(args.iter_mut());
            }
        }

        OperandIter { operands }
    }
}

pub struct OperandIter<'instr> {
    operands: Vec<&'instr mut Register>,
}

impl<'instr> Iterator for OperandIter<'instr> {
    type Item = &'instr mut Register;

    fn next(&mut self) -> Option<Self::Item> {
        self.operands.pop()
    }
}
