use std::fmt::{self, Debug, Formatter};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
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

#[derive(PartialEq, Eq, Clone, Copy, Default)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub slice: &'a str,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, slice: &'a str) -> Self {
        Self { kind, slice }
    }
}

impl Debug for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}: {}]", self.kind, self.slice)
    }
}
