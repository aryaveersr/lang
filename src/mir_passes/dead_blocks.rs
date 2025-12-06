use crate::{
    mir::{BlockID, MirFun, MirModule, Term},
    mir_passes::MirPass,
};

#[derive(Default)]
pub struct DeadBlocks {
    marked_blocks: Vec<BlockID>,
}

impl MirPass for DeadBlocks {
    fn run(&mut self, module: &mut MirModule) {
        for fun in &mut module.funs {
            self.marked_blocks.clear();
            self.mark_fun(fun);
            self.sweep_fun(fun);
        }
    }
}

impl DeadBlocks {
    fn sweep_fun(&self, fun: &mut MirFun) {
        fun.blocks.retain(|b| self.marked_blocks.contains(&b.id));
    }

    fn mark_fun(&mut self, fun: &mut MirFun) {
        if let Some(next) = fun.blocks.first() {
            self.mark_block(fun, next.id);
        }
    }

    fn mark_block(&mut self, fun: &mut MirFun, id: BlockID) {
        let block = fun.get_block_mut(id);
        self.marked_blocks.push(id);

        match block.term {
            Some(Term::Branch {
                then_block,
                else_block,
                ..
            }) => {
                self.mark_block(fun, then_block);
                self.mark_block(fun, else_block);
            }

            Some(Term::Jump { block }) => self.mark_block(fun, block),

            _ => {}
        }
    }
}
