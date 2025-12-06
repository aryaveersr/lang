use serde::Serialize;
use std::fmt::{self, Debug, Display, Formatter};

use crate::position::Position;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Serialize)]
pub enum TokenKind {
    // Single-characters.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Colon,
    Comma,
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
pub struct Token<'src> {
    pub kind: TokenKind,
    pub slice: &'src str,
    pub pos: Position,
}

impl<'src> Token<'src> {
    pub fn new(kind: TokenKind, slice: &'src str, pos: Position) -> Self {
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
            Self::LeftParen => write!(f, "("),
            Self::RightParen => write!(f, ")"),
            Self::LeftBrace => write!(f, "{{"),
            Self::RightBrace => write!(f, "}}"),
            Self::Semicolon => write!(f, ";"),
            Self::Colon => write!(f, ":"),
            Self::Comma => write!(f, ","),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Equal => write!(f, "="),
            Self::EqualEqual => write!(f, "=="),
            Self::Lesser => write!(f, "<"),
            Self::LesserEqual => write!(f, "<="),
            Self::Greater => write!(f, ">"),
            Self::GreaterEqual => write!(f, ">="),
            Self::Not => write!(f, "!"),
            Self::NotEqual => write!(f, "!="),
            Self::Identifier => write!(f, "identifier"),
            Self::Numeric => write!(f, "numeric"),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
            Self::Fun => write!(f, "fun"),
            Self::Return => write!(f, "return"),
            Self::If => write!(f, "if"),
            Self::Else => write!(f, "else"),
            Self::Loop => write!(f, "loop"),
            Self::While => write!(f, "while"),
            Self::Break => write!(f, "break"),
            Self::Let => write!(f, "let"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}
