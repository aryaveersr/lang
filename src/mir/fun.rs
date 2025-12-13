use crate::{cfg::Cfg, mir::MirFun};

impl MirFun {
    pub fn new(name: String) -> Self {
        Self {
            name,
            params: Vec::new(),
            blocks: Vec::new(),
            cfg: Cfg::default(),
            return_ty: None,
        }
    }
}
