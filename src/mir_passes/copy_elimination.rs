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
            let try_rename = |value: &mut ValueID| {
                if let Some(new_value) = self.worklist.get(value) {
                    *value = *new_value;
                }
            };

            for phi in &mut block.phis {
                try_rename(&mut phi.dest);

                for (_, src) in &mut phi.srcs {
                    try_rename(src);
                }
            }

            for instr in &mut block.instrs {
                try_rename(&mut instr.dest);
                instr.operands().for_each(try_rename);
            }

            if let Some(term) = &mut block.term
                && let Some(operand) = term.operand()
            {
                try_rename(operand);
            }
        }
    }
}
