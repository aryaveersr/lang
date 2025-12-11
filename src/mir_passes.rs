use crate::{
    mir::MirModule,
    mir_passes::{
        remove_empty_blocks::remove_empty_blocks,
        remove_unreachable_blocks::remove_unreachable_blocks, sync_block_ids::sync_block_ids,
    },
};

mod remove_empty_blocks;
mod remove_unreachable_blocks;
mod rename_blocks;
mod sync_block_ids;

pub fn run_passes(mir: &mut MirModule) {
    for fun in &mut mir.funs {
        remove_unreachable_blocks(fun);
        remove_empty_blocks(fun);
        sync_block_ids(fun);
    }
}
