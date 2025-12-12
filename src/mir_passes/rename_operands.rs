use std::collections::HashMap;

use crate::mir::{InstrKind, MirFun, Term, Value};

pub fn rename_operands(fun: &mut MirFun, mapping: &HashMap<Value, Value>) {
    let rename = |value: &mut Value| {
        if let Some(new_value) = mapping.get(value) {
            *value = *new_value;
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
