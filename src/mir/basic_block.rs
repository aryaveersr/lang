use crate::mir::{BasicBlock, BlockID, Phi, ValueID};

impl BasicBlock {
    pub fn new(id: BlockID) -> Self {
        Self {
            id,
            phis: Vec::new(),
            instrs: Vec::new(),
            term: None,
        }
    }

    pub fn get_phi_mut(&mut self, dest: ValueID) -> &mut Phi {
        self.phis
            .iter_mut()
            .find(|phi| phi.dest == dest)
            .expect("No phi exists for the destination.")
    }
}
