use crate::mir::ValueID;

impl ValueID {
    pub fn temporary(id: u32) -> Self {
        Self(0, id)
    }

    pub fn variable(variable: u32, generation: u32) -> Self {
        debug_assert!(variable != 0);
        Self(variable, generation)
    }

    pub fn is_temporary(&self) -> bool {
        self.0 == 0
    }

    pub fn is_variable(&self) -> bool {
        self.0 != 0
    }

    pub fn get_variable(&self) -> u32 {
        debug_assert!(self.0 != 0);
        self.0
    }

    pub fn get_generation(&self) -> u32 {
        debug_assert!(self.0 != 0);
        self.1
    }
}
