use crate::mir::{BlockID, MirFun, Term};

pub fn remove_unreachable_blocks(fun: &mut MirFun) {
    let mut marked_blocks = Vec::new();

    if let Some(first_block) = fun.blocks.first() {
        mark_block(fun, &mut marked_blocks, first_block.id);
    }

    fun.blocks.retain(|block| marked_blocks.contains(&block.id));

    for block in &mut fun.blocks {
        for phi in &mut block.phis {
            phi.srcs.retain(|(src, _)| marked_blocks.contains(src));
        }
    }
}

fn mark_block(fun: &mut MirFun, marked_blocks: &mut Vec<BlockID>, id: BlockID) {
    if marked_blocks.contains(&id) {
        return;
    }

    marked_blocks.push(id);

    match fun.blocks[id].term {
        Some(Term::Return { .. }) | None => {}
        Some(Term::Jump { target }) => mark_block(fun, marked_blocks, target),

        Some(Term::Branch {
            then_block,
            else_block,
            ..
        }) => {
            mark_block(fun, marked_blocks, then_block);
            mark_block(fun, marked_blocks, else_block);
        }
    }
}
