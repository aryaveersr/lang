use itertools::Itertools as _;
use std::collections::HashMap;

use crate::{
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
    active_block: BlockID,
    sealed_blocks: Vec<BlockID>,
    incomplete_phis: Vec<Vec<(VarID, Reg)>>,
    var_uses: Vec<HashMap<(VarID, Gen), Operand>>,
    var_gens: HashMap<VarID, Gen>,
    next_var: usize,
    next_reg: usize,
}

impl MirBuilder {
    pub fn new(name: String) -> Self {
        let mut fun = MirFun::new(name);

        fun.blocks.push(BasicBlock::new(BlockID(0)));

        Self {
            fun,
            active_block: BlockID(0),
            sealed_blocks: Vec::new(),
            incomplete_phis: vec![Vec::new()],
            var_uses: vec![HashMap::new()],
            var_gens: HashMap::new(),
            next_var: 0,
            next_reg: 0,
        }
    }

    pub fn create_block(&mut self) -> BlockID {
        let id = BlockID(self.fun.blocks.len());

        self.fun.blocks.push(BasicBlock::new(id));
        self.incomplete_phis.push(Vec::new());
        self.var_uses.push(HashMap::new());

        id
    }

    pub fn seal_block(&mut self, id: BlockID) {
        let incomplete_phis = std::mem::take(&mut self.incomplete_phis[id]);

        for (var_id, dest) in incomplete_phis {
            self.add_phi_operands(id, var_id, dest);
        }

        self.sealed_blocks.push(id);
    }

    pub fn set_active_block(&mut self, id: BlockID) {
        self.active_block = id;
    }

    pub fn declare_var(&mut self) -> VarID {
        let var_id = VarID(self.next_var);
        self.next_var += 1;

        self.var_gens.insert(var_id, 0);

        var_id
    }

    pub fn assign_var(&mut self, var_id: VarID, value: Value) {
        let value = self.resolve_value(value);
        let genn = self.fresh_var_gen(var_id);

        self.var_uses[self.active_block].insert((var_id, genn), value);
    }

    pub fn has_terminator(&self) -> bool {
        self.fun.blocks[self.active_block.0].term.is_some()
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
        self.fun.cfg.add_edge(self.active_block, target);

        self.build_term(Term::Jump { target });
    }

    pub fn build_branch(&mut self, cond: Value, then_block: BlockID, else_block: BlockID) {
        let cond = self.resolve_value(cond);

        self.fun.cfg.add_edge(self.active_block, then_block);
        self.fun.cfg.add_edge(self.active_block, else_block);

        self.build_term(Term::Branch {
            cond,
            then_block,
            else_block,
        });
    }

    pub fn build_return(&mut self, value: Option<Value>) {
        let value = value.map(|value| self.resolve_value(value));

        self.build_term(Term::Return { value });
    }

    pub fn finish(mut self, return_ty: Option<MirType>) -> MirFun {
        let incomplete_phis = std::mem::take(&mut self.incomplete_phis);

        for (id, incomplete_phis) in incomplete_phis.into_iter().enumerate() {
            for (var_id, dest) in incomplete_phis {
                self.add_phi_operands(BlockID(id), var_id, dest);
            }
        }

        self.fun.return_ty = return_ty;
        self.fun
    }

    fn resolve_value(&mut self, value: Value) -> Operand {
        match value {
            Value::Operand(operand) => operand,
            Value::Variable(var_id) => self.read_var(self.active_block, var_id).unwrap().1,
        }
    }

    fn build_instr(&mut self, kind: InstrKind) -> Value {
        kind.try_fold()
            .unwrap_or_else(|| {
                let dest = self.fresh_reg();

                self.fun.blocks[self.active_block]
                    .instrs
                    .push(Instr { dest, kind });

                Operand::Reg(dest)
            })
            .into()
    }

    fn build_term(&mut self, term: Term) {
        self.fun.blocks[self.active_block].term = Some(term);
    }

    fn fresh_reg(&mut self) -> Reg {
        self.next_reg += 1;
        Reg(self.next_reg - 1)
    }

    fn fresh_var_gen(&mut self, var_id: VarID) -> Gen {
        let genn = self.var_gens[&var_id];
        self.var_gens.insert(var_id, genn + 1);
        genn
    }

    fn add_phi_operands(&mut self, block: BlockID, var_id: VarID, dest: Reg) {
        let preds = self.fun.cfg.predecessors(block);

        for pred in preds {
            if let Some(src) = self.read_var(pred, var_id) {
                self.fun.blocks[block].get_phi_mut(dest).srcs.push(src);
            }
        }
    }

    fn read_var(&mut self, block: BlockID, var_id: VarID) -> Option<(BlockID, Operand)> {
        if let Some((_, operand)) = self.var_uses[block]
            .iter()
            .filter(|&(&(v, _), _)| v == var_id)
            .max_by_key(|&(&(_, g), _)| g)
        {
            return Some((block, *operand));
        }

        if self.sealed_blocks.contains(&block) {
            let preds = self.fun.cfg.predecessors(block);

            if preds.len() == 1 {
                self.read_var(preds[0], var_id)
            } else {
                let srcs = preds
                    .iter()
                    .filter_map(|pred| self.read_var(*pred, var_id))
                    .collect_vec();

                if srcs.is_empty() {
                    None
                } else if srcs.iter().all(|src| src.1 == srcs[0].1) {
                    Some((block, srcs[0].1))
                } else {
                    let genn = self.fresh_var_gen(var_id);
                    let dest = self.fresh_reg();

                    self.var_uses[block].insert((var_id, genn), Operand::Reg(dest));
                    self.fun.blocks[block].phis.push(Phi { dest, srcs });

                    Some((block, Operand::Reg(dest)))
                }
            }
        } else {
            let genn = self.fresh_var_gen(var_id);
            let dest = self.fresh_reg();

            self.incomplete_phis[block].push((var_id, dest));
            self.var_uses[block].insert((var_id, genn), Operand::Reg(dest));
            self.fun.blocks[block].phis.push(Phi {
                dest,
                srcs: Vec::new(),
            });

            Some((block, Operand::Reg(dest)))
        }
    }
}
