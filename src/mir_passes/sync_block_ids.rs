use crate::{
    mir::{BlockID, MirFun},
    mir_passes::rename_blocks::rename_blocks,
};

pub fn sync_block_ids(fun: &mut MirFun) {
    let renamed_blocks = fun
        .blocks
        .iter()
        .enumerate()
        .map(|(i, block)| (block.id, BlockID(i)))
        .filter(|(a, b)| a != b)
        .collect();

    rename_blocks(fun, &renamed_blocks);
}
