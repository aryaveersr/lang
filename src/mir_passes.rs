use crate::{
    mir::{MirFun, MirModule},
    mir_passes::{
        cfg::Cfg, copy_elimination::CopyElimination, rename_blocks::RenameBlocks,
        unreachable_blocks::UnreachableBlocks, variables::VarUseAnalyzer,
    },
};

mod cfg;
mod copy_elimination;
mod rename_blocks;
mod unreachable_blocks;
mod variables;

trait MirPass<'fun, T = ()> {
    fn run(fun: &'fun mut MirFun) -> T;
}

pub fn run_passes(module: &mut MirModule) {
    for fun in &mut module.funs {
        UnreachableBlocks::run(fun);
        RenameBlocks::run(fun);
        CopyElimination::run(fun);

        let _cfg = Cfg::from(&*fun);
        let _var_use_info = VarUseAnalyzer::run(fun);
    }
}
