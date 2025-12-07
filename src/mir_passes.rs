use crate::{
    mir::{MirFun, MirModule},
    mir_passes::{rename_blocks::RenameBlocks, unreachable_blocks::UnreachableBlocks},
};

mod rename_blocks;
mod unreachable_blocks;

trait MirPass<'a> {
    fn run(fun: &'a mut MirFun);
}

pub fn run_passes(module: &mut MirModule) {
    for fun in &mut module.funs {
        UnreachableBlocks::run(fun);
        RenameBlocks::run(fun);
    }
}
