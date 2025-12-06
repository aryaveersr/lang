use crate::{
    mir::{BasicBlock, BlockID, Instr, InstrKind, MirFun, Phi, Term, ValueID},
    ops::{BinOp, UnOp},
};

pub struct Builder {
    pub fun: MirFun,
    block: Option<BlockID>,
}

impl Builder {
    pub fn new(name: String) -> Self {
        Self {
            fun: MirFun::new(name),
            block: None,
        }
    }

    pub fn create_block(&mut self) -> BlockID {
        let id = BlockID(self.fun.next_block);
        self.fun.next_block += 1;
        self.fun.blocks.push(BasicBlock::new(id));
        id
    }

    pub fn set_active_block(&mut self, block: BlockID) {
        self.block = Some(block);
    }

    pub fn active_block(&mut self) -> &mut BasicBlock {
        let id = self.block.expect("No active block set.");
        self.fun
            .blocks
            .iter_mut()
            .find(|b| b.id == id)
            .expect("Invalid block id.")
    }

    pub fn fresh_value(&mut self) -> ValueID {
        let id = ValueID(self.fun.next_value);
        self.fun.next_value += 1;
        id
    }

    pub fn finish(self) -> MirFun {
        self.fun
    }

    pub fn add_instr(&mut self, instr: Instr) {
        self.active_block().instrs.push(instr);
    }

    pub fn add_term(&mut self, term: Term) {
        self.active_block().term = Some(term);
    }

    pub fn add_const_bool(&mut self, value: bool) -> ValueID {
        let dest = self.fresh_value();
        self.add_instr(Instr {
            dest,
            kind: InstrKind::ConstBool { value },
        });

        dest
    }

    pub fn add_const_num(&mut self, value: i32) -> ValueID {
        let dest = self.fresh_value();
        self.add_instr(Instr {
            dest,
            kind: InstrKind::ConstNum { value },
        });

        dest
    }

    pub fn add_copy(&mut self, src: ValueID) -> ValueID {
        let dest = self.fresh_value();
        self.add_instr(Instr {
            dest,
            kind: InstrKind::Copy { src },
        });

        dest
    }

    pub fn add_unary(&mut self, op: UnOp, arg: ValueID) -> ValueID {
        let dest = self.fresh_value();
        self.add_instr(Instr {
            dest,
            kind: InstrKind::Unary { op, arg },
        });

        dest
    }

    pub fn add_binary(&mut self, op: BinOp, lhs: ValueID, rhs: ValueID) -> ValueID {
        let dest = self.fresh_value();
        self.add_instr(Instr {
            dest,
            kind: InstrKind::Binary { op, lhs, rhs },
        });

        dest
    }

    pub fn add_call(&mut self, name: String, args: Vec<ValueID>) -> ValueID {
        let dest = self.fresh_value();
        self.add_instr(Instr {
            dest,
            kind: InstrKind::Call { name, args },
        });

        dest
    }

    pub fn add_jump(&mut self, block: BlockID) {
        self.add_term(Term::Jump { block });
    }

    pub fn add_branch(&mut self, cond: ValueID, then_block: BlockID, else_block: BlockID) {
        self.add_term(Term::Branch {
            cond,
            then_block,
            else_block,
        });
    }

    pub fn add_return(&mut self, value: Option<ValueID>) {
        self.add_term(Term::Return { value });
    }

    pub fn add_phi(&mut self) -> ValueID {
        let dest = self.fresh_value();
        self.active_block().phis.push(Phi::new(dest));
        dest
    }

    pub fn add_phi_src(&mut self, phi: ValueID, value: ValueID, block: BlockID) {
        let phi = self
            .active_block()
            .phis
            .iter_mut()
            .find(|p| p.dest == phi)
            .expect("Invalid phi node.");

        phi.srcs.push((block, value));
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new(String::new())
    }
}
