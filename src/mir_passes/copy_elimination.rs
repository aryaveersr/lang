use std::collections::HashMap;

use crate::{
    mir::{InstrKind, MirFun, Term, ValueID},
    mir_passes::MirPass,
};

pub struct CopyElimination<'fun> {
    fun: &'fun mut MirFun,
    worklist: HashMap<ValueID, ValueID>,
}

impl<'fun> MirPass<'fun> for CopyElimination<'fun> {
    fn run(fun: &'fun mut MirFun) {
        Self::new(fun).eliminate_copies();
    }
}

impl<'fun> CopyElimination<'fun> {
    fn new(fun: &'fun mut MirFun) -> Self {
        Self {
            fun,
            worklist: HashMap::new(),
        }
    }

    fn eliminate_copies(&mut self) {
        self.find_copies();
        self.rename_uses();
    }

    fn find_copies(&mut self) {
        for block in &mut self.fun.blocks {
            block.instrs.retain(|instr| match &instr.kind {
                InstrKind::Copy { src } => {
                    self.worklist.insert(*src, instr.dest);
                    false
                }

                _ => true,
            });
        }
    }

    fn rename_uses(&mut self) {
        for block in &mut self.fun.blocks {
            let try_rename = |value: &mut ValueID| {
                if let Some(new_value) = self.worklist.get(value) {
                    *value = *new_value;
                }
            };

            for instr in &mut block.instrs {
                try_rename(&mut instr.dest);

                match &mut instr.kind {
                    InstrKind::ConstBool { .. } | InstrKind::ConstNum { .. } => {}

                    InstrKind::Unary { arg: src, .. } | InstrKind::Copy { src } => {
                        try_rename(src);
                    }

                    InstrKind::Binary { lhs, rhs, .. } => {
                        try_rename(lhs);
                        try_rename(rhs);
                    }

                    InstrKind::Call { args, .. } => {
                        for arg in args {
                            try_rename(arg);
                        }
                    }
                }
            }

            if let Some(term) = &mut block.term {
                match term {
                    Term::Return { value: None } | Term::Jump { .. } => {}

                    Term::Return { value: Some(value) } | Term::Branch { cond: value, .. } => {
                        try_rename(value);
                    }
                }
            }
        }
    }
}
