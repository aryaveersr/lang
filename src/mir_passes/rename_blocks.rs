use crate::{
    mir::{BlockID, MirFun, Term},
    mir_passes::MirPass,
};

pub struct RenameBlocks<'fun> {
    fun: &'fun mut MirFun,
    ids: Vec<BlockID>,
}

impl<'fun> MirPass<'fun> for RenameBlocks<'fun> {
    fn run(fun: &'fun mut MirFun) {
        Self::new(fun).rename_blocks();
    }
}

impl<'fun> RenameBlocks<'fun> {
    fn new(fun: &'fun mut MirFun) -> Self {
        Self {
            fun,
            ids: Vec::new(),
        }
    }

    fn rename_blocks(&mut self) {
        self.populate_mapping();
        self.update_terminators();
    }

    fn populate_mapping(&mut self) {
        for block in &self.fun.blocks {
            self.ids.push(block.id);
        }
    }

    fn update_terminators(&mut self) {
        let new_id = |id: &BlockID| BlockID(self.ids.iter().position(|x| x == id).unwrap());

        for block in &mut self.fun.blocks {
            if let Some(term) = &mut block.term {
                match term {
                    Term::Branch {
                        then_block,
                        else_block,
                        ..
                    } => {
                        *then_block = new_id(then_block);
                        *else_block = new_id(else_block);
                    }

                    Term::Jump { block } => *block = new_id(block),
                    Term::Return { .. } => {}
                }
            }
        }
    }
}
