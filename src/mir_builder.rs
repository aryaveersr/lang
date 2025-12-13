use itertools::Itertools as _;
use std::collections::HashMap;

use crate::{
    cfg::Cfg,
    mir::{BasicBlock, BlockID, Instr, InstrKind, MirFun, MirType, Operand, Phi, Reg, Term},
    mir_builder::value::Value,
    ops::{BinOp, UnOp},
};

pub mod value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VarID(usize);
type Gen = usize;

pub struct MirBuilder {
    fun: MirFun,
    active_id: BlockID,
    sealed_blocks: Vec<BlockID>,
    definitions: Vec<Vec<(VarID, Gen, Reg)>>,
    incomplete_phis: Vec<(BlockID, VarID, Reg)>,
    consts: HashMap<Reg, Operand>,
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
        let mut incomplete_phis = Vec::new();

        self.incomplete_phis.retain(|&(block_id, var_id, dest)| {
            if block_id == id {
                incomplete_phis.push((var_id, dest));
                false
            } else {
                true
            }
        });

        for (var_id, dest) in incomplete_phis {
            self.add_phi_operands(id, var_id, dest);
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
        let id = Reg(self.next_temp);
        self.next_temp += 1;
        id
    }

    pub fn finish(mut self) -> MirFun {
        for (block_id, var_id, dest) in std::mem::take(&mut self.incomplete_phis) {
            self.add_phi_operands(block_id, var_id, dest);
        }

        if self.fun.blocks[self.active_id].term.is_none() {
            self.build_return(None);
        }

        self.fun
    }

    pub fn declare_var(&mut self) -> VarID {
        let id = VarID(self.next_var_id);
        self.next_var_id += 1;

        self.var_gens.insert(id, 0);

        id
    }

    pub fn assign_var(&mut self, var_id: VarID, value: Value) {
        let value = self.resolve_value(value);
        let reg = self.fresh_temp();
        let genn = self.fresh_gen(var_id);

        self.definitions[self.active_id].push((var_id, genn, reg));
        self.consts.insert(reg, value);
    }

    pub fn is_terminated(&self) -> bool {
        self.fun.blocks[self.active_id].term.is_some()
    }

    pub fn build_unary(&mut self, op: UnOp, arg: Value) -> Value {
        let arg = self.resolve_value(arg);

        self.build_instr(InstrKind::Unary { op, arg })
    }

    pub fn build_binary(&mut self, op: BinOp, lhs: Value, rhs: Value) -> Value {
        let lhs = self.resolve_value(lhs);
        let rhs = self.resolve_value(rhs);

        self.build_instr(InstrKind::Binary { op, lhs, rhs })
    }

    pub fn build_call(&mut self, name: String, args: Vec<Value>) -> Value {
        let args = args
            .into_iter()
            .map(|arg| self.resolve_value(arg))
            .collect();

        self.build_instr(InstrKind::Call { name, args })
    }

    pub fn build_jump(&mut self, target: BlockID) {
        self.mark_flow(target);
        self.push_term(Term::Jump { target });
    }

    pub fn build_branch(&mut self, cond: Value, then_block: BlockID, else_block: BlockID) {
        let cond = self.resolve_value(cond);

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

    pub fn build_return(&mut self, value: Option<Value>) {
        let value = value.map(|value| self.resolve_value(value));

        self.push_term(Term::Return { value });
    }

    fn add_phi_operands(&mut self, id: BlockID, var_id: VarID, dest: Reg) {
        let preds = self.cfg.predecessors(id);

        for pred in preds {
            if let Some(src) = self.read_var(var_id, pred) {
                self.fun.blocks[id].get_phi_mut(dest).srcs.push(src);
            }
        }
    }

    fn read_var(&mut self, var_id: VarID, block: BlockID) -> Option<(BlockID, Operand)> {
        if let Some((_, _, reg)) = self.definitions[block]
            .iter()
            .filter(|&&(id, _, _)| id == var_id)
            .max_by_key(|&&(_, genn, _)| genn)
        {
            if let Some(value) = self.consts.get(reg) {
                return Some((block, *value));
            }

            return Some((block, Operand::Reg(*reg)));
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
                    let genn = self.fresh_gen(var_id);
                    let dest = self.fresh_temp();

                    self.definitions[block].push((var_id, genn, dest));
                    self.fun.blocks[block].phis.push(Phi { dest, srcs });

                    Some((block, Operand::Reg(dest)))
                }
            }
        } else {
            let genn = self.fresh_gen(var_id);
            let dest = self.fresh_temp();

            self.incomplete_phis.push((block, var_id, dest));
            self.definitions[block].push((var_id, genn, dest));
            self.fun.blocks[block].phis.push(Phi { dest, srcs: vec![] });

            Some((block, Operand::Reg(dest)))
        }
    }

    fn fresh_gen(&mut self, var_id: VarID) -> Gen {
        *self.var_gens.get_mut(&var_id).unwrap() += 1;
        self.var_gens[&var_id] - 1
    }

    fn mark_flow(&mut self, to: BlockID) {
        assert!(!self.sealed_blocks.contains(&to));
        self.cfg.add_edge(self.active_id, to);
    }

    fn build_instr(&mut self, instr_kind: InstrKind) -> Value {
        if let Some(value) = instr_kind.try_fold() {
            value.into()
        } else {
            let dest = self.fresh_temp();

            self.fun.blocks[self.active_id].instrs.push(Instr {
                dest,
                kind: instr_kind,
            });

            Value::reg(dest)
        }
    }

    fn push_term(&mut self, term: Term) {
        self.fun.blocks[self.active_id].term = Some(term);
    }

    fn resolve_value(&mut self, value: Value) -> Operand {
        match value {
            Value::Operand(operand) => operand,
            Value::Variable(var_id) => self.read_var(var_id, self.active_id).unwrap().1,
        }
    }
}
