use crate::mir::{BasicBlock, BlockID, MirFun, MirType};

impl MirFun {
    pub fn new(name: String) -> Self {
        Self {
            name,
            params: Vec::new(),
            blocks: Vec::new(),
            return_ty: None,
        }
    }

    pub fn get_block(&self, id: BlockID) -> &BasicBlock {
        self.blocks
            .iter()
            .find(|block| block.id == id)
            .expect("Invalid Block ID.")
    }

    pub fn get_block_mut(&mut self, id: BlockID) -> &mut BasicBlock {
        self.blocks
            .iter_mut()
            .find(|block| block.id == id)
            .expect("Invalid Block ID.")
    }

    #[must_use]
    pub fn with_return_type(mut self, ty: Option<MirType>) -> Self {
        self.return_ty = ty;
        self
    }
}
