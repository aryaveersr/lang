use std::collections::HashMap;

use crate::mir::{InstrKind, MirFun, Operand, Term};

pub fn rename_operands(fun: &mut MirFun, mapping: &HashMap<Operand, Operand>) {
    let rename = |operand: &mut Operand| {
        if let Some(new_operand) = mapping.get(operand) {
            *operand = *new_operand;
        }
    };

    for block in &mut fun.blocks {
        for instr in &mut block.instrs {
            match &mut instr.kind {
                InstrKind::Unary { arg, .. } => rename(arg),

                InstrKind::Binary { lhs, rhs, .. } => {
                    rename(lhs);
                    rename(rhs);
                }

                InstrKind::Call { args, .. } => {
                    for arg in args {
                        rename(arg);
                    }
                }
            }
        }

        if let Some(term) = &mut block.term {
            match term {
                Term::Jump { .. } => {}
                Term::Branch { cond, .. } => rename(cond),

                Term::Return { value } => {
                    if let Some(value) = value {
                        rename(value);
                    }
                }
            }
        }
    }
}
