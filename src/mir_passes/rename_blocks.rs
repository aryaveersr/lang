use std::collections::HashMap;

use crate::mir::{BlockID, MirFun, Term};

pub fn rename_blocks(fun: &mut MirFun, mapping: &HashMap<BlockID, BlockID>) {
    for block in &mut fun.blocks {
        let rename = |id: &mut BlockID| {
            if let Some(new_id) = mapping.get(id) {
                *id = *new_id;
            }
        };

        rename(&mut block.id);

        match &mut block.term {
            Some(Term::Return { .. }) | None => {}
            Some(Term::Jump { target }) => rename(target),

            Some(Term::Branch {
                then_block,
                else_block,
                ..
            }) => {
                rename(then_block);
                rename(else_block);
            }
        }

        for phi in &mut block.phis {
            for (src, _) in &mut phi.srcs {
                rename(src);
            }
        }
    }
}
