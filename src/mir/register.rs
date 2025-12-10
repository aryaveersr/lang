use crate::mir::Register;

impl Register {
    pub fn temporary(id: usize) -> Self {
        Self(0, id)
    }

    pub fn variable(variable: usize, generation: usize) -> Self {
        debug_assert!(variable != 0);
        Self(variable, generation)
    }

    pub fn is_temporary(&self) -> bool {
        self.0 == 0
    }

    pub fn is_variable(&self) -> bool {
        self.0 != 0
    }

    pub fn get_variable(&self) -> usize {
        debug_assert!(self.0 != 0);
        self.0
    }

    pub fn get_generation(&self) -> usize {
        debug_assert!(self.0 != 0);
        self.1
    }
}
