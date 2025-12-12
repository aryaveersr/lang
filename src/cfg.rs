use crate::mir::BlockID;

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
}
