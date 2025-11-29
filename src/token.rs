use crate::position::Position;
use serde::Serialize;
use std::fmt::{self, Debug, Display, Formatter};

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
    Unknown,
}

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Default)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub slice: &'a str,
    pub pos: Position,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, slice: &'a str, pos: Position) -> Self {
        Self { kind, slice, pos }
    }
}

impl Debug for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}: {} at {}]", self.kind, self.slice, self.pos)
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::LeftBrace => write!(f, "{{"),
            TokenKind::RightBrace => write!(f, "}}"),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Equal => write!(f, "="),
            TokenKind::EqualEqual => write!(f, "=="),
            TokenKind::Lesser => write!(f, "<"),
            TokenKind::LesserEqual => write!(f, "<="),
            TokenKind::Greater => write!(f, ">"),
            TokenKind::GreaterEqual => write!(f, ">="),
            TokenKind::Not => write!(f, "!"),
            TokenKind::NotEqual => write!(f, "!="),
            TokenKind::Identifier => write!(f, "identifier"),
            TokenKind::Numeric => write!(f, "numeric"),
            TokenKind::True => write!(f, "true"),
            TokenKind::False => write!(f, "false"),
            TokenKind::And => write!(f, "and"),
            TokenKind::Or => write!(f, "or"),
            TokenKind::Fun => write!(f, "fun"),
            TokenKind::Return => write!(f, "return"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::Loop => write!(f, "loop"),
            TokenKind::While => write!(f, "while"),
            TokenKind::Break => write!(f, "break"),
            TokenKind::Let => write!(f, "let"),
            TokenKind::Unknown => write!(f, "unknown"),
        }
    }
}
