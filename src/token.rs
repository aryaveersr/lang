use serde::Serialize;
use std::fmt::{self, Debug, Formatter};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Serialize)]
pub enum TokenKind {
    // Single-characters.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Colon,
    Plus,
    Minus,
    Star,
    Slash,

    // One or two characters.
    Equal,
    EqualEqual,
    Lesser,
    LesserEqual,
    Greater,
    GreaterEqual,
    Not,
    NotEqual,

    // Variable length.
    Identifier,
    Numeric,

    // Keywords.
    True,
    False,
    And,
    Or,
    Fun,
    Return,
    If,
    Else,
    Loop,
    While,
    Break,
    Let,

    // Error.
    #[default]
    Invalid,
}

#[derive(PartialEq, Eq, Clone, Copy, Serialize)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub slice: &'a str,
    pub line: usize,
    pub column: usize,
}

impl<'a> Default for Token<'a> {
    fn default() -> Self {
        Self {
            kind: TokenKind::default(),
            slice: "",
            line: 1,
            column: 1,
        }
    }
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, slice: &'a str, line: usize, column: usize) -> Self {
        Self {
            kind,
            slice,
            line,
            column,
        }
    }
}

impl Debug for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{:?}: {} at {}:{}]",
            self.kind, self.slice, self.line, self.column
        )
    }
}
