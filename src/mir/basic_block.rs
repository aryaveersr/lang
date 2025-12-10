use crate::mir::{BasicBlock, BlockID, Phi, Register};

impl BasicBlock {
    pub fn new(id: BlockID) -> Self {
        Self {
            id,
            phis: Vec::new(),
            instrs: Vec::new(),
            term: None,
        }
    }

    pub fn get_phi_mut(&mut self, dest: Register) -> &mut Phi {
        self.phis
            .iter_mut()
            .find(|phi| phi.dest == dest)
            .expect("No phi exists for the destination.")
    }

    pub fn values_mut<F: FnMut(&mut Register)>(&mut self, mut f: F) {
        for phi in &mut self.phis {
            f(&mut phi.dest);

            for (_, src) in &mut phi.srcs {
                f(src);
            }
        }

        for instr in &mut self.instrs {
            f(&mut instr.dest);

            for op in instr.operands() {
                f(op);
            }
        }

        if let Some(term) = &mut self.term
            && let Some(operand) = term.operand()
        {
            f(operand);
        }
    }
}
