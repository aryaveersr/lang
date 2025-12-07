use std::fmt::Display;

use crate::mir::{BlockID, MirFun, Term};

#[derive(Default, Debug, Clone)]
pub struct Cfg {
    edges: Vec<(BlockID, BlockID)>,
}

impl Cfg {
    pub fn add_edge(&mut self, from: BlockID, to: BlockID) {
        self.edges.push((from, to));
    }

    pub fn predecessors(&self, id: BlockID) -> Vec<BlockID> {
        self.edges
            .iter()
            .filter(|(_, to)| *to == id)
            .map(|(from, _)| *from)
            .collect()
    }

    pub fn successors(&self, id: BlockID) -> Vec<BlockID> {
        self.edges
            .iter()
            .filter(|(from, _)| *from == id)
            .map(|(_, to)| *to)
            .collect()
    }
}

impl From<&MirFun> for Cfg {
    fn from(value: &MirFun) -> Self {
        let mut cfg = Cfg::default();

        for block in &value.blocks {
            if let Some(term) = &block.term {
                match term {
                    Term::Branch {
                        then_block,
                        else_block,
                        ..
                    } => {
                        cfg.add_edge(block.id, *then_block);
                        cfg.add_edge(block.id, *else_block);
                    }

                    Term::Jump { block: next } => {
                        cfg.add_edge(block.id, *next);
                    }

                    Term::Return { .. } => {}
                }
            }
        }

        cfg
    }
}

impl Display for Cfg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CFG:")?;

        for (from, to) in &self.edges {
            writeln!(f, "{} -> {}", from, to)?;
        }

        Ok(())
    }
}
