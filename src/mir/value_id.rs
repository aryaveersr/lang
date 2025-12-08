use crate::mir::ValueID;

impl ValueID {
    pub fn temporary(id: u32) -> Self {
        ValueID(0, id)
    }

    pub fn variable(variable: u32, generation: u32) -> Self {
        ValueID(variable, generation)
    }

    pub fn is_temporary(&self) -> bool {
        self.0 == 0
    }

    pub fn is_variable(&self) -> bool {
        self.0 != 0
    }

    pub fn get_var(&self) -> usize {
        self.0 as usize
    }
}
