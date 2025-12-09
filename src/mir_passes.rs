use crate::{mir::MirModule, mir_passes::unreachable_blocks::UnreachableBlocks};

mod unreachable_blocks;

pub fn run_passes(mir: &mut MirModule) {
    for fun in &mut mir.funs {
        UnreachableBlocks::run(fun);
    }
}
