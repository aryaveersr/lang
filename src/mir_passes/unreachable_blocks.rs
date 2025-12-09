use std::collections::HashMap;

use crate::mir::{BlockID, MirFun, Term};

pub struct UnreachableBlocks<'fun> {
    fun: &'fun mut MirFun,
    marked_blocks: Vec<BlockID>,
    renamed_blocks: HashMap<BlockID, BlockID>,
}

impl<'fun> UnreachableBlocks<'fun> {
    pub fn run(fun: &'fun mut MirFun) {
        Self::new(fun).remove_unreachable_blocks();
    }

    fn new(fun: &'fun mut MirFun) -> Self {
        Self {
            fun,
            marked_blocks: Vec::new(),
            renamed_blocks: HashMap::new(),
        }
    }

    fn remove_unreachable_blocks(&mut self) {
        if let Some(first_block) = self.fun.blocks.first() {
            self.mark_block(first_block.id);
        }

        self.sweep();
        self.remove_phi_sources();
        self.rename_blocks();
    }

    fn mark_block(&mut self, id: BlockID) {
        if self.marked_blocks.contains(&id) {
            return;
        }

        let block = self.fun.get_block_mut(id);

        if block.instrs.len() == 0
            && block.phis.len() == 0
            && let Some(Term::Jump { target }) = block.term
        {
            self.renamed_blocks.insert(id, target);
            self.mark_block(target);
            return;
        }

        self.marked_blocks.push(id);

        match block.term {
            Some(Term::Branch {
                then_block,
                else_block,
                ..
            }) => {
                self.mark_block(then_block);
                self.mark_block(else_block);
            }

            Some(Term::Jump { target }) => self.mark_block(target),

            Some(Term::Return { .. }) | None => {}
        }
    }

    fn sweep(&mut self) {
        self.fun
            .blocks
            .retain(|b| self.marked_blocks.contains(&b.id));
    }

    fn remove_phi_sources(&mut self) {
        for block in &mut self.fun.blocks {
            for phi in &mut block.phis {
                phi.srcs.retain(|(src, _)| self.marked_blocks.contains(src));
            }
        }
    }

    fn rename_blocks(&mut self) {
        for block in &mut self.fun.blocks {
            let try_rename = |id: &mut BlockID| {
                if let Some(target) = self.renamed_blocks.get(&id) {
                    *id = *target;
                }
            };

            if let Some(term) = &mut block.term {
                match term {
                    Term::Return { .. } => {}
                    Term::Jump { target } => try_rename(target),

                    Term::Branch {
                        then_block,
                        else_block,
                        ..
                    } => {
                        try_rename(then_block);
                        try_rename(else_block);
                    }
                }
            }

            for phi in &mut block.phis {
                for (src, _) in &mut phi.srcs {
                    try_rename(src);
                }
            }
        }
    }
}
