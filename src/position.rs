use std::fmt::{self, Display, Formatter};

use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn newline(&mut self) {
        self.line += 1;
        self.column = 1;
    }

    pub fn take_char(&mut self, c: char) {
        if c == '\n' {
            self.newline();
        } else {
            self.column += 1;
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl Default for Position {
    fn default() -> Self {
        Self { line: 1, column: 1 }
    }
}
