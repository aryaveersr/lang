use itertools::Itertools as _;
use std::collections::HashMap;

use crate::{
    mir::{
        BasicBlock, BlockID, Gen, Instr, InstrKind, MirFun, MirType, Phi, Reg, Term, Value, VarID,
        cfg::Cfg,
    },
    ops::{BinOp, UnOp},
};

pub struct Builder {
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

        if self.active_block().term.is_none() {
            self.build_return(None);
        }

        self.fun
    }

    pub fn declare_var(&mut self, value: Value) -> Value {
        let var = Value::Reg(Reg::new_var(self.next_var_id, 0));

        self.var_gens.insert(self.next_var_id, 0);
        self.assign_var(var, value);
        self.next_var_id += 1;

        var
    }

    pub fn assign_var(&mut self, var: Value, value: Value) {
        let new_id = self.fresh_var(var.as_reg().unwrap().get_var_id().unwrap());

        self.definitions[self.active_id].push(new_id);
        self.consts
            .insert(new_id, self.as_const(value).unwrap_or(value));
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
            return Some(
                self.consts
                    .get(reg)
                    .map_or_else(|| (block, (*reg).into()), |value| (block, *value)),
            );
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

                    Some((block, dest.into()))
                }
            }
        } else {
            let dest = self.fresh_var(var_id);

            self.incomplete_phis.push(dest);
            self.definitions[block].push(dest);
            self.fun.blocks[block].phis.push(Phi { dest, srcs: vec![] });

            Some((block, dest.into()))
        }
    }
}

impl Builder {
    fn add_flow(&mut self, to: BlockID) {
        debug_assert!(!self.sealed_blocks.contains(&to));
        self.cfg.add_edge(self.active_id, to);
    }

    fn active_block(&mut self) -> &mut BasicBlock {
        &mut self.fun.blocks[self.active_id]
    }

    fn push_instr(&mut self, instr: Instr) {
        self.active_block().instrs.push(instr);
    }

    fn push_term(&mut self, term: Term) {
        self.active_block().term = Some(term);
    }

    fn use_value(&mut self, value: Value) -> Value {
        if let Some(var_id) = value.as_reg().and_then(|reg| reg.get_var_id())
            && let Some(new_value) = self.read_var(var_id, self.active_id)
        {
            new_value.1
        } else {
            value
        }
    }

    fn as_const(&self, value: Value) -> Option<Value> {
        value
            .as_reg()
            .map_or(Some(value), |reg| self.consts.get(&reg).copied())
    }

    pub fn is_terminated(&self) -> bool {
        self.fun.blocks[self.active_id].term.is_some()
    }

    pub fn build_unary(&mut self, op: UnOp, arg: Value) -> Value {
        let arg = self.use_value(arg);

        self.as_const(arg).map_or_else(
            || {
                let dest = self.fresh_temp();

                self.push_instr(Instr {
                    dest,
                    kind: InstrKind::Unary { op, arg },
                });

                Value::Reg(dest)
            },
            |arg| match op {
                UnOp::Negate => Value::Num(-arg.as_num()),
                UnOp::Not => Value::Bool(!arg.as_bool()),
            },
        )
    }

    pub fn build_binary(&mut self, op: BinOp, lhs: Value, rhs: Value) -> Value {
        let lhs = self.use_value(lhs);
        let rhs = self.use_value(rhs);

        if let Some(lhs) = self.as_const(lhs)
            && let Some(rhs) = self.as_const(rhs)
        {
            match op {
                BinOp::Add => Value::Num(lhs.as_num() + rhs.as_num()),
                BinOp::Sub => Value::Num(lhs.as_num() - rhs.as_num()),
                BinOp::Mul => Value::Num(lhs.as_num() * rhs.as_num()),
                BinOp::Div => Value::Num(lhs.as_num() / rhs.as_num()),
                BinOp::Eq => Value::Bool(lhs == rhs),
                BinOp::NotEq => Value::Bool(lhs != rhs),
                BinOp::Lesser => Value::Bool(lhs.as_num() < rhs.as_num()),
                BinOp::LesserEq => Value::Bool(lhs.as_num() <= rhs.as_num()),
                BinOp::Greater => Value::Bool(lhs.as_num() > rhs.as_num()),
                BinOp::GreaterEq => Value::Bool(lhs.as_num() >= rhs.as_num()),
                BinOp::And => Value::Bool(lhs.as_bool() && rhs.as_bool()),
                BinOp::Or => Value::Bool(lhs.as_bool() || rhs.as_bool()),
            }
        } else {
            let dest = self.fresh_temp();

            self.push_instr(Instr {
                dest,
                kind: InstrKind::Binary { op, lhs, rhs },
            });

            Value::Reg(dest)
        }
    }

    pub fn build_call(&mut self, name: String, args: Vec<Value>) -> Value {
        let dest = self.fresh_temp();
        let args = args.into_iter().map(|arg| self.use_value(arg)).collect();

        self.push_instr(Instr {
            dest,
            kind: InstrKind::Call { name, args },
        });

        Value::Reg(dest)
    }

    pub fn build_jump(&mut self, target: BlockID) {
        self.add_flow(target);

        self.push_term(Term::Jump { target });
    }

    pub fn build_branch(&mut self, cond: Value, then_block: BlockID, else_block: BlockID) {
        let cond = self.use_value(cond);

        if let Some(cond) = self.as_const(cond) {
            if cond.as_bool() {
                self.build_jump(then_block);
            } else {
                self.build_jump(else_block);
            }
        } else {
            self.add_flow(then_block);
            self.add_flow(else_block);

            self.push_term(Term::Branch {
                cond,
                then_block,
                else_block,
            });
        }
    }

    pub fn build_return(&mut self, value: Option<Value>) {
        let value = value.map(|value| self.use_value(value));

        self.push_term(Term::Return { value });
    }
}
