use crate::{
    mir::{BlockID, MirFun, Term},
    mir_passes::MirPass,
};

pub struct UnreachableBlocks<'fun> {
    fun: &'fun mut MirFun,
    marked_blocks: Vec<BlockID>,
}

impl<'fun> MirPass<'fun> for UnreachableBlocks<'fun> {
    fn run(fun: &'fun mut MirFun) {
        Self::new(fun).remove_unreachable_blocks();
    }
}

impl<'fun> UnreachableBlocks<'fun> {
    fn new(fun: &'fun mut MirFun) -> Self {
        Self {
            fun,
            marked_blocks: Vec::new(),
        }
    }

    fn remove_unreachable_blocks(&mut self) {
        if let Some(first_block) = self.fun.blocks.first() {
            self.mark_block(first_block.id);
        }

        self.sweep();
    }

    fn mark_block(&mut self, id: BlockID) {
        if self.marked_blocks.contains(&id) {
            return;
        }

        self.marked_blocks.push(id);

        match self.fun.get_block_mut(id).term {
            Some(Term::Branch {
                then_block,
                else_block,
                ..
            }) => {
                self.mark_block(then_block);
                self.mark_block(else_block);
            }

            Some(Term::Jump { block }) => self.mark_block(block),

            Some(Term::Return { .. }) | None => {}
        }
    }

    fn sweep(&mut self) {
        self.fun
            .blocks
            .retain(|b| self.marked_blocks.contains(&b.id));
    }
}
