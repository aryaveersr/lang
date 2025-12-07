use crate::{
    mir::{MirFun, MirModule},
    mir_passes::{rename_blocks::RenameBlocks, unreachable_blocks::UnreachableBlocks},
};

mod rename_blocks;
mod unreachable_blocks;

trait MirPass<'fun> {
    fn run(fun: &'fun mut MirFun);
}

pub fn run_passes(module: &mut MirModule) {
    for fun in &mut module.funs {
        UnreachableBlocks::run(fun);
        RenameBlocks::run(fun);
    }
}
