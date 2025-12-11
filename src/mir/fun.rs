use crate::mir::MirFun;

impl MirFun {
    pub fn new(name: String) -> Self {
        Self {
            name,
            params: Vec::new(),
            blocks: Vec::new(),
            return_ty: None,
        }
    }
}
