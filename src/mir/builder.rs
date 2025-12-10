use itertools::Itertools as _;
use std::collections::HashMap;

use crate::{
    mir::{BasicBlock, BlockID, Gen, Instr, InstrKind, MirFun, Phi, Reg, Term, VarID, cfg::Cfg},
    ops::{BinOp, UnOp},
};

pub struct Builder {
    fun: MirFun,
    active_id: BlockID,
    sealed_blocks: Vec<BlockID>,
    definitions: Vec<Vec<Reg>>,
    incomplete_phis: HashMap<BlockID, Vec<(VarID, Reg)>>,
    var_gens: HashMap<VarID, Gen>,
    cfg: Cfg,
    next_temp: usize,
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
            var_gens: HashMap::new(),
            cfg: Cfg::default(),
            next_temp: 0,
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

    pub fn fresh_temp(&mut self) -> Reg {
        let id = Reg::Temp(self.next_temp);
        self.next_temp += 1;
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

    pub fn declare_var(&mut self, var_id: VarID, value: Reg) -> Reg {
        self.var_gens.insert(var_id, 1);
        let new_id = Reg::Var(var_id, 0);

        self.definitions[self.active_id].push(new_id);

        self.push_instr(Instr {
            dest: new_id,
            kind: InstrKind::Copy { src: value },
        });

        new_id
    }

    pub fn assign_var(&mut self, reg: Reg, value: Reg) {
        let (var_id, _) = reg.as_var().unwrap();
        let new_id = self.fresh_var(var_id);

        self.definitions[self.active_id].push(new_id);

        self.push_instr(Instr {
            dest: new_id,
            kind: InstrKind::Copy { src: value },
        });
    }

    fn fresh_var(&mut self, var_id: VarID) -> Reg {
        let new_id = Reg::Var(var_id, self.var_gens[&var_id]);
        self.var_gens.entry(var_id).and_modify(|g| *g += 1);

        new_id
    }

    fn add_phi_operands(&mut self, id: BlockID, var_id: VarID, dest: Reg) {
        let preds = self.cfg.predecessors(id);

        for pred in preds {
            if let Some(src) = self.read_var(var_id, pred) {
                self.fun.get_block_mut(id).get_phi_mut(dest).srcs.push(src);
            }
        }
    }

    fn read_var(&mut self, var_id: VarID, block: BlockID) -> Option<(BlockID, Reg)> {
        if let Some(value) = self.definitions[block]
            .iter()
            .find(|v| v.get_var_id() == Some(var_id))
        {
            return Some((block, *value));
        }

        if self.sealed_blocks.contains(&block) {
            let preds = self.cfg.predecessors(block);

            if preds.len() == 1 {
                self.read_var(var_id, preds[0])
            } else {
                let srcs = preds
                    .iter()
                    .filter_map(|pred| self.read_var(var_id, *pred))
                    .collect_vec();

                if srcs.is_empty() {
                    None
                } else if srcs.len() == 1 {
                    Some((block, srcs[0].1))
                } else {
                    let dest = self.fresh_var(var_id);

                    self.definitions[block].push(dest);
                    self.fun.get_block_mut(block).phis.push(Phi { dest, srcs });

                    Some((block, dest))
                }
            }
        } else {
            let dest = self.fresh_var(var_id);
            self.definitions[block].push(dest);
            self.fun
                .get_block_mut(block)
                .phis
                .push(Phi { dest, srcs: vec![] });

            self.incomplete_phis
                .entry(block)
                .or_default()
                .push((var_id, dest));

            Some((block, dest))
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
        for value in instr.operands().filter(|v| v.is_var()) {
            if let Some(new_value) = self.read_var(value.get_var_id().unwrap(), self.active_id) {
                *value = new_value.1;
            }
        }

        self.active_block().instrs.push(instr);
    }

    fn push_term(&mut self, mut term: Term) {
        if let Some(value) = term.operand()
            && let Some(var_id) = value.get_var_id()
            && let Some(new_value) = self.read_var(var_id, self.active_id)
        {
            *value = new_value.1;
        }

        self.active_block().term = Some(term);
    }

    pub fn is_terminated(&self) -> bool {
        self.fun.get_block(self.active_id).term.is_some()
    }

    pub fn build_const_bool(&mut self, value: bool) -> Reg {
        let dest = self.fresh_temp();

        self.push_instr(Instr {
            dest,
            kind: InstrKind::ConstBool { value },
        });

        dest
    }

    pub fn build_const_num(&mut self, value: i32) -> Reg {
        let dest = self.fresh_temp();

        self.push_instr(Instr {
            dest,
            kind: InstrKind::ConstNum { value },
        });

        dest
    }

    pub fn build_unary(&mut self, op: UnOp, arg: Reg) -> Reg {
        let dest = self.fresh_temp();

        self.push_instr(Instr {
            dest,
            kind: InstrKind::Unary { op, arg },
        });

        dest
    }

    pub fn build_binary(&mut self, op: BinOp, lhs: Reg, rhs: Reg) -> Reg {
        let dest = self.fresh_temp();

        self.push_instr(Instr {
            dest,
            kind: InstrKind::Binary { op, lhs, rhs },
        });

        dest
    }

    pub fn build_call(&mut self, name: String, args: Vec<Reg>) -> Reg {
        let dest = self.fresh_temp();

        self.push_instr(Instr {
            dest,
            kind: InstrKind::Call { name, args },
        });

        dest
    }

    pub fn build_jump(&mut self, target: BlockID) {
        self.add_flow(target);

        self.push_term(Term::Jump { target });
    }

    pub fn build_branch(&mut self, cond: Reg, then_block: BlockID, else_block: BlockID) {
        self.add_flow(then_block);
        self.add_flow(else_block);

        self.push_term(Term::Branch {
            cond,
            then_block,
            else_block,
        });
    }

    pub fn build_return(&mut self, value: Option<Reg>) {
        self.push_term(Term::Return { value });
    }
}
