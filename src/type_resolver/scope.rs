use std::collections::HashMap;

use crate::hir::HirType;

#[derive(Default)]
pub struct Scope {
    scopes: Vec<HashMap<String, HirType>>,
}

impl Scope {
    pub fn create(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop(&mut self) -> HashMap<String, HirType> {
        self.scopes.pop().expect("scope stack underflow")
    }

    pub fn set(&mut self, name: &str, value: &HirType) {
        let last = self.scopes.last_mut().expect("scope stack empty");
        last.insert(name.to_owned(), value.to_owned());
    }

    pub fn get(&self, name: &str) -> Option<&HirType> {
        self.scopes.iter().rev().find_map(|s| s.get(name))
    }
}
