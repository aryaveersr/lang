use crate::mir::{BasicBlock, BlockID, MirFun, Phi, ValueID};

impl MirFun {
    pub fn new(name: String) -> Self {
        Self {
            name,
            params: Vec::new(),
            blocks: Vec::new(),
            return_ty: None,
            next_block: 0,
            next_value: 0,
        }
    }

    pub fn get_block(&self, id: BlockID) -> &BasicBlock {
        self.blocks.iter().find(|block| block.id == id).unwrap()
    }

    pub fn get_block_mut(&mut self, id: BlockID) -> &mut BasicBlock {
        self.blocks.iter_mut().find(|block| block.id == id).unwrap()
    }
}

impl BasicBlock {
    pub fn new(id: BlockID) -> Self {
        Self {
            id,
            phis: Vec::new(),
            instrs: Vec::new(),
            term: None,
        }
    }
}

impl Phi {
    pub fn new(dest: ValueID) -> Self {
        Self {
            dest,
            srcs: Vec::new(),
        }
    }
}
