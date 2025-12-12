use itertools::Itertools as _;
use std::collections::HashMap;

use crate::{
    cfg::Cfg,
    mir::{
        BasicBlock, BlockID, Gen, Instr, InstrKind, MirFun, MirType, Phi, Reg, Term, Value, VarID,
    },
    mir_builder::operand::Operand,
    ops::{BinOp, UnOp},
};

mod const_folding;
pub mod operand;

pub struct MirBuilder {
    fun: MirFun,
    active_id: BlockID,
    sealed_blocks: Vec<BlockID>,
    definitions: Vec<Vec<Reg>>,
    incomplete_phis: Vec<Reg>,
    consts: HashMap<Reg, Value>,
    cfg: Cfg,
    var_gens: HashMap<VarID, Gen>,
    next_temp: usize,
    next_var_id: usize,
}

impl MirBuilder {
    pub fn new(name: String) -> Self {
        let entry = BlockID(0);

        let mut fun = MirFun::new(name);
        fun.blocks.push(BasicBlock::new(entry));

        Self {
            fun,
            active_id: entry,
            sealed_blocks: vec![entry],
            definitions: vec![Vec::new()],
            incomplete_phis: Vec::new(),
            consts: HashMap::new(),
            var_gens: HashMap::new(),
            cfg: Cfg::default(),
            next_temp: 0,
            next_var_id: 0,
        }
    }

    pub fn create_block(&mut self) -> BlockID {
        let id = BlockID(self.fun.blocks.len());

        self.fun.blocks.push(BasicBlock::new(id));
        self.definitions.push(Vec::new());

        id
    }

    pub fn seal_block(&mut self, id: BlockID) {
        let mut dests = Vec::new();

        self.incomplete_phis.retain(|dest| {
            if self.definitions[id].contains(dest) {
                dests.push(*dest);
                false
            } else {
                true
            }
        });

        for dest in dests {
            self.add_phi_operands(id, dest);
        }

        self.sealed_blocks.push(id);
    }

    pub fn set_active(&mut self, id: BlockID) {
        self.active_id = id;
    }

    pub fn set_return_type(&mut self, ty: Option<MirType>) {
        self.fun.return_ty = ty;
    }

    pub fn fresh_temp(&mut self) -> Reg {
        let id = Reg::Temp(self.next_temp);
        self.next_temp += 1;
        id
    }

    pub fn finish(mut self) -> MirFun {
        for dest in std::mem::take(&mut self.incomplete_phis) {
            let id = self
                .definitions
                .iter()
                .position(|defs| defs.contains(&dest))
                .unwrap();

            self.add_phi_operands(BlockID(id), dest);
        }

        if self.fun.blocks[self.active_id].term.is_none() {
            self.build_return(None);
        }

        self.fun
    }

    pub fn declare_var(&mut self) -> VarID {
        let id = self.next_var_id;
        self.next_var_id += 1;

        self.var_gens.insert(id, 0);

        id
    }

    pub fn assign_var(&mut self, var_id: VarID, value: Operand) {
        let reg = self.fresh_var(var_id);
        let value = self.resolve_operand(value);

        self.definitions[self.active_id].push(reg);
        self.consts.insert(reg, value);
    }

    pub fn is_terminated(&self) -> bool {
        self.fun.blocks[self.active_id].term.is_some()
    }

    pub fn build_unary(&mut self, op: UnOp, arg: Operand) -> Value {
        let arg = self.resolve_operand(arg);

        if arg.is_const() {
            op.fold(arg)
        } else {
            let dest = self.fresh_temp();

            self.push_instr(Instr {
                dest,
                kind: InstrKind::Unary { op, arg },
            });

            Value::Reg(dest)
        }
    }

    pub fn build_binary(&mut self, op: BinOp, lhs: Operand, rhs: Operand) -> Value {
        let lhs = self.resolve_operand(lhs);
        let rhs = self.resolve_operand(rhs);

        if lhs.is_const() && rhs.is_const() {
            op.fold(lhs, rhs)
        } else {
            let dest = self.fresh_temp();

            self.push_instr(Instr {
                dest,
                kind: InstrKind::Binary { op, lhs, rhs },
            });

            Value::Reg(dest)
        }
    }

    pub fn build_call(&mut self, name: String, args: Vec<Operand>) -> Value {
        let args = args
            .into_iter()
            .map(|arg| self.resolve_operand(arg))
            .collect();

        let dest = self.fresh_temp();

        self.push_instr(Instr {
            dest,
            kind: InstrKind::Call { name, args },
        });

        Value::Reg(dest)
    }

    pub fn build_jump(&mut self, target: BlockID) {
        self.mark_flow(target);
        self.push_term(Term::Jump { target });
    }

    pub fn build_branch(&mut self, cond: Operand, then_block: BlockID, else_block: BlockID) {
        let cond = self.resolve_operand(cond);

        if cond.is_const() {
            self.build_jump(if cond.as_bool() {
                then_block
            } else {
                else_block
            });
        } else {
            self.mark_flow(then_block);
            self.mark_flow(else_block);

            self.push_term(Term::Branch {
                cond,
                then_block,
                else_block,
            });
        }
    }

    pub fn build_return(&mut self, value: Option<Operand>) {
        let value = value.map(|operand| self.resolve_operand(operand));

        self.push_term(Term::Return { value });
    }

    fn fresh_var(&mut self, var_id: VarID) -> Reg {
        let new_id = Reg::new_var(var_id, self.var_gens[&var_id]);
        self.var_gens.entry(var_id).and_modify(|g| *g += 1);

        new_id
    }

    fn add_phi_operands(&mut self, id: BlockID, dest: Reg) {
        let preds = self.cfg.predecessors(id);
        let var_id = dest.get_var_id().unwrap();

        for pred in preds {
            if let Some(src) = self.read_var(var_id, pred) {
                self.fun.blocks[id].get_phi_mut(dest).srcs.push(src);
            }
        }
    }

    fn read_var(&mut self, var_id: VarID, block: BlockID) -> Option<(BlockID, Value)> {
        if let Some(reg) = self.definitions[block]
            .iter()
            .rev()
            .find(|v| v.get_var_id() == Some(var_id))
        {
            if let Some(value) = self.consts.get(reg) {
                return Some((block, *value));
            }

            return Some((block, Value::Reg(*reg)));
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
                } else if srcs.iter().all(|src| src.1 == srcs[0].1) {
                    Some((block, srcs[0].1))
                } else {
                    let dest = self.fresh_var(var_id);

                    self.definitions[block].push(dest);
                    self.fun.blocks[block].phis.push(Phi { dest, srcs });

                    Some((block, Value::Reg(dest)))
                }
            }
        } else {
            let dest = self.fresh_var(var_id);

            self.incomplete_phis.push(dest);
            self.definitions[block].push(dest);
            self.fun.blocks[block].phis.push(Phi { dest, srcs: vec![] });

            Some((block, Value::Reg(dest)))
        }
    }

    fn mark_flow(&mut self, to: BlockID) {
        assert!(!self.sealed_blocks.contains(&to));
        self.cfg.add_edge(self.active_id, to);
    }

    fn push_instr(&mut self, instr: Instr) {
        self.fun.blocks[self.active_id].instrs.push(instr);
    }

    fn push_term(&mut self, term: Term) {
        self.fun.blocks[self.active_id].term = Some(term);
    }

    fn resolve_operand(&mut self, operand: Operand) -> Value {
        match operand {
            Operand::Value(value) => value,
            Operand::Variable(var_id) => self.read_var(var_id, self.active_id).unwrap().1,
        }
    }
}
