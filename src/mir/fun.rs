use crate::mir::{MirFun, MirType};

impl MirFun {
    pub fn new(name: String) -> Self {
        Self {
            name,
            params: Vec::new(),
            blocks: Vec::new(),
            return_ty: None,
        }
    }

    #[must_use]
    pub fn with_return_type(mut self, ty: Option<MirType>) -> Self {
        self.return_ty = ty;
        self
    }
}
