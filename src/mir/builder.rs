use itertools::Itertools as _;
use std::collections::HashMap;

use crate::{
    mir::{BasicBlock, BlockID, Instr, InstrKind, MirFun, Phi, Term, ValueID, cfg::Cfg},
    ops::{BinOp, UnOp},
};

pub type Variable = u32;
type Generation = u32;

pub struct Builder {
    fun: MirFun,
    active_id: BlockID,
    sealed_blocks: Vec<BlockID>,
    definitions: Vec<Vec<ValueID>>,
    incomplete_phis: HashMap<BlockID, Vec<(Variable, ValueID)>>,
    var_generations: HashMap<Variable, Generation>,
    cfg: Cfg,
    next_temporary: u32,
}

impl Builder {
    pub fn new(name: String) -> Self {
        let entry = BlockID(0);

        let mut fun = MirFun::new(name);
        fun.blocks.push(BasicBlock::new(entry));

        Self {
            fun,
            active_id: entry,
            sealed_blocks: vec![entry],
            definitions: vec![Vec::new()],
            incomplete_phis: HashMap::new(),
            var_generations: HashMap::new(),
            cfg: Cfg::default(),
            next_temporary: 0,
        }
    }

    pub fn create_block(&mut self) -> BlockID {
        let id = BlockID(self.fun.blocks.len());

        self.fun.blocks.push(BasicBlock::new(id));
        self.definitions.push(Vec::new());

        id
    }

    pub fn seal_block(&mut self, id: BlockID) {
        if let Some(phis) = self.incomplete_phis.remove(&id) {
            for (var, dest) in phis {
                self.add_phi_operands(id, var, dest);
            }
        }

        self.sealed_blocks.push(id);
    }

    pub fn set_active(&mut self, id: BlockID) {
        self.active_id = id;
    }

    pub fn fresh_temp(&mut self) -> ValueID {
        let id = ValueID::temporary(self.next_temporary);
        self.next_temporary += 1;
        id
    }

    pub fn finish(mut self) -> MirFun {
        for (id, phis) in self.incomplete_phis.drain().collect_vec() {
            for (var, dest) in phis {
                self.add_phi_operands(id, var, dest);
            }
        }

        if self.active_block().term.is_none() {
            self.build_return(None);
        }

        self.fun
    }

    pub fn declare_variable(&mut self, variable: Variable, value: ValueID) -> ValueID {
        debug_assert!(variable != 0);

        self.var_generations.insert(variable, 1);
        let new_id = ValueID::variable(variable, 0);

        self.definitions[self.active_id].push(new_id);

        self.push_instr(Instr {
            dest: new_id,
            kind: InstrKind::Copy { src: value },
        });

        new_id
    }

    pub fn assign_variable(&mut self, variable: Variable, value: ValueID) {
        debug_assert!(variable != 0);

        let new_id = self.fresh_variable(variable);

        self.definitions[self.active_id].push(new_id);

        self.push_instr(Instr {
            dest: new_id,
            kind: InstrKind::Copy { src: value },
        });
    }

    fn fresh_variable(&mut self, variable: Variable) -> ValueID {
        let new_id = ValueID::variable(variable, self.var_generations[&variable]);
        self.var_generations.entry(variable).and_modify(|g| *g += 1);

        new_id
    }

    fn add_phi_operands(&mut self, id: BlockID, variable: Variable, dest: ValueID) {
        let preds = self.cfg.predecessors(id);

        for pred in preds {
            let src = self.read_variable(variable, pred);
            self.fun.get_block_mut(id).get_phi_mut(dest).srcs.push(src);
        }
    }

    fn read_variable(&mut self, variable: Variable, block: BlockID) -> (BlockID, ValueID) {
        if let Some(value) = self.definitions[block]
            .iter()
            .find(|v| v.get_variable() == variable)
        {
            return (block, *value);
        }

        if self.sealed_blocks.contains(&block) {
            let preds = self.cfg.predecessors(block);

            if preds.len() == 1 {
                self.read_variable(variable, preds[0])
            } else {
                let mut srcs = Vec::new();
                let dest = self.fresh_variable(variable);

                self.definitions[block].push(dest);

                for pred in preds {
                    srcs.push(self.read_variable(variable, pred));
                }

                self.fun.get_block_mut(block).phis.push(Phi { dest, srcs });
                (block, dest)
            }
        } else {
            let dest = self.fresh_variable(variable);
            self.definitions[block].push(dest);
            self.fun
                .get_block_mut(block)
                .phis
                .push(Phi { dest, srcs: vec![] });

            self.incomplete_phis
                .entry(block)
                .or_default()
                .push((variable, dest));

            (block, dest)
        }
    }
}

impl Builder {
    fn add_flow(&mut self, to: BlockID) {
        debug_assert!(!self.sealed_blocks.contains(&to));
        self.cfg.add_edge(self.active_id, to);
    }

    fn active_block(&mut self) -> &mut BasicBlock {
        self.fun.get_block_mut(self.active_id)
    }

    fn push_instr(&mut self, mut instr: Instr) {
        for value in instr.operands().filter(|v| v.is_variable()) {
            *value = self.read_variable(value.get_variable(), self.active_id).1;
        }

        self.active_block().instrs.push(instr);
    }

    fn push_term(&mut self, mut term: Term) {
        if let Some(value) = term.operands()
            && value.is_variable()
        {
            *value = self.read_variable(value.get_variable(), self.active_id).1;
        }

        self.active_block().term = Some(term);
    }

    pub fn is_terminated(&self) -> bool {
        self.fun.get_block(self.active_id).term.is_some()
    }

    pub fn build_const_bool(&mut self, value: bool) -> ValueID {
        let dest = self.fresh_temp();

        self.push_instr(Instr {
            dest,
            kind: InstrKind::ConstBool { value },
        });

        dest
    }

    pub fn build_const_num(&mut self, value: i32) -> ValueID {
        let dest = self.fresh_temp();

        self.push_instr(Instr {
            dest,
            kind: InstrKind::ConstNum { value },
        });

        dest
    }

    pub fn build_unary(&mut self, op: UnOp, arg: ValueID) -> ValueID {
        let dest = self.fresh_temp();

        self.push_instr(Instr {
            dest,
            kind: InstrKind::Unary { op, arg },
        });

        dest
    }

    pub fn build_binary(&mut self, op: BinOp, lhs: ValueID, rhs: ValueID) -> ValueID {
        let dest = self.fresh_temp();

        self.push_instr(Instr {
            dest,
            kind: InstrKind::Binary { op, lhs, rhs },
        });

        dest
    }

    pub fn build_call(&mut self, name: String, args: Vec<ValueID>) -> ValueID {
        let dest = self.fresh_temp();

        self.push_instr(Instr {
            dest,
            kind: InstrKind::Call { name, args },
        });

        dest
    }

    pub fn build_jump(&mut self, block: BlockID) {
        self.add_flow(block);

        self.push_term(Term::Jump { block });
    }

    pub fn build_branch(&mut self, cond: ValueID, then_block: BlockID, else_block: BlockID) {
        self.add_flow(then_block);
        self.add_flow(else_block);

        self.push_term(Term::Branch {
            cond,
            then_block,
            else_block,
        });
    }

    pub fn build_return(&mut self, value: Option<ValueID>) {
        self.push_term(Term::Return { value });
    }
}
