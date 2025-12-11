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
            let target = if let Some(new_target) = renamed_blocks.get(&target) {
                *new_target
            } else {
                target
            };

            renamed_blocks.insert(block.id, target);
        }
    }

    fun.blocks
        .retain(|block| !renamed_blocks.contains_key(&block.id));

    rename_blocks(fun, &renamed_blocks);
}
