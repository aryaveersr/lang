use std::collections::HashMap;

pub struct Scope<T> {
    scopes: Vec<HashMap<String, T>>,
}

impl<T: Clone> Scope<T> {
    pub fn create(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop(&mut self) -> HashMap<String, T> {
        self.scopes.pop().unwrap()
    }

    pub fn set(&mut self, name: &str, value: &T) {
        let last = self.scopes.last_mut().unwrap();
        last.insert(name.to_owned(), value.to_owned());
    }

    pub fn get(&self, name: &str) -> Option<&T> {
        self.scopes.iter().rev().find_map(|s| s.get(name))
    }
}

impl<T> Default for Scope<T> {
    fn default() -> Self {
        Self {
            scopes: Vec::default(),
        }
    }
}
