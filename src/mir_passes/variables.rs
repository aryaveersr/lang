use std::collections::HashSet;

use crate::{
    mir::{InstrKind, MirFun, ValueID},
    mir_passes::MirPass,
};

pub struct VarUseInfo {
    pub defined_vars: Vec<HashSet<ValueID>>,
    pub upward_exposed_vars: Vec<HashSet<ValueID>>,
}

impl VarUseInfo {
    fn empty(block_count: usize) -> Self {
        Self {
            defined_vars: vec![HashSet::new(); block_count],
            upward_exposed_vars: vec![HashSet::new(); block_count],
        }
    }
}

pub struct VarUseAnalyzer<'fun> {
    fun: &'fun MirFun,
    info: VarUseInfo,
}

impl<'fun> MirPass<'fun, VarUseInfo> for VarUseAnalyzer<'fun> {
    fn run(fun: &'fun mut MirFun) -> VarUseInfo {
        Self::new(fun).analyze()
    }
}

impl<'fun> VarUseAnalyzer<'fun> {
    fn new(fun: &'fun MirFun) -> Self {
        Self {
            info: VarUseInfo::empty(fun.blocks.len()),
            fun,
        }
    }

    fn analyze(mut self) -> VarUseInfo {
        self.analyze_var_use();
        self.info
    }

    fn analyze_var_use(&mut self) {
        for block in &self.fun.blocks {
            let defined_vars = &mut self.info.defined_vars[block.id];
            let upward_exposed_vars = &mut self.info.upward_exposed_vars[block.id];

            for instr in &block.instrs {
                let mut mark_used = |value: &ValueID| {
                    if !defined_vars.contains(value) {
                        upward_exposed_vars.insert(*value);
                    }
                };

                match &instr.kind {
                    InstrKind::ConstBool { .. } | InstrKind::ConstNum { .. } => {}
                    InstrKind::Copy { src } | InstrKind::Unary { arg: src, .. } => mark_used(src),

                    InstrKind::Binary { lhs, rhs, .. } => {
                        mark_used(lhs);
                        mark_used(rhs);
                    }

                    InstrKind::Call { args, .. } => {
                        for arg in args {
                            mark_used(arg);
                        }
                    }
                }

                defined_vars.insert(instr.dest);
            }
        }
    }
}
