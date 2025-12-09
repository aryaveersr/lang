use crate::{
    mir::MirModule,
    mir_passes::{copy_elimination::CopyElimination, unreachable_blocks::UnreachableBlocks},
};

mod copy_elimination;
mod unreachable_blocks;

pub fn run_passes(mir: &mut MirModule) {
    for fun in &mut mir.funs {
        UnreachableBlocks::run(fun);
        CopyElimination::run(fun);
    }
}
