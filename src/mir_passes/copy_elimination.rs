use std::collections::HashMap;

use crate::mir::{InstrKind, MirFun, ValueID};

pub struct CopyElimination<'fun> {
    fun: &'fun mut MirFun,
    worklist: HashMap<ValueID, ValueID>,
}

impl<'fun> CopyElimination<'fun> {
    pub fn run(fun: &'fun mut MirFun) {
        Self::new(fun).eliminate_copies();
    }

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
                    if let Some(new_dest) = self.worklist.get(&instr.dest) {
                        self.worklist.insert(*src, *new_dest);
                    } else {
                        self.worklist.insert(*src, instr.dest);
                    }

                    false
                }

                _ => true,
            });
        }
    }

    fn rename_uses(&mut self) {
        for block in &mut self.fun.blocks {
            block.values_mut(|value| {
                if let Some(new_value) = self.worklist.get(value) {
                    *value = *new_value;
                }
            });
        }
    }
}
