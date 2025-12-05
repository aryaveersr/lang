use crate::mir::{BasicBlock, BlockID, MirFun, Phi, ValueID};

impl MirFun {
    pub fn new(name: String) -> Self {
        Self {
            name,
            blocks: Vec::new(),
            return_ty: None,
            next_block: 0,
            next_value: 0,
        }
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
