use std::ops::{Index, IndexMut};

use crate::mir::BlockID;

impl<T> Index<BlockID> for Vec<T> {
    type Output = T;

    fn index(&self, index: BlockID) -> &Self::Output {
        self.index(index.0)
    }
}

impl<T> IndexMut<BlockID> for Vec<T> {
    fn index_mut(&mut self, index: BlockID) -> &mut Self::Output {
        self.index_mut(index.0)
    }
}
