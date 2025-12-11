use std::collections::HashMap;

use crate::{
    mir::{BlockID, MirFun, Term},
    mir_passes::rename_blocks::rename_blocks,
};

pub fn remove_empty_blocks(fun: &mut MirFun) {
    let mut renamed_blocks = HashMap::new();

    for block in &mut fun.blocks {
        if block.instrs.is_empty()
            && block.phis.is_empty()
            && block.id != BlockID(0)
            && let Some(Term::Jump { target }) = block.term
        {
            let renamed_to = renamed_blocks.get(&target).copied().unwrap_or(target);

            renamed_blocks.insert(block.id, renamed_to);
            renamed_blocks
                .values_mut()
                .filter(|id| **id == block.id)
                .for_each(|id| *id = renamed_to);
        }
    }

    fun.blocks
        .retain(|block| !renamed_blocks.contains_key(&block.id));

    rename_blocks(fun, &renamed_blocks);
}
