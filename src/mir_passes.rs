use crate::{
    mir::{MirFun, MirModule},
    mir_passes::{
        cfg::Cfg, rename_blocks::RenameBlocks, unreachable_blocks::UnreachableBlocks,
        variables::VarUseAnalyzer,
    },
};

mod cfg;
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

        let _cfg = Cfg::from(&*fun);
        let _var_use_info = VarUseAnalyzer::run(fun);
    }
}
