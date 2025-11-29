use crate::{
    position::Position,
    token::{Token, TokenKind},
};
use serde::Serialize;
use thiserror::Error;

pub type ParseResult<T> = std::result::Result<T, ParseError>;

#[derive(Error, Debug, Clone, PartialEq, Eq, Serialize)]
pub enum ParseError {
    #[error("Missing main function.")]
    MissingMainFunction,

    #[error("Break statement outside of loop at {pos}.")]
    BreakOutsideLoop { pos: Position },

    #[error("Unexpected end of file, expected {expected}.")]
    UnexpectedEOF { expected: String },

    #[error("Number too large at {pos}.")]
    NumberTooLarge { pos: Position },

    #[error("Invalid expression: {found} at {pos}.")]
    InvalidExpr { found: TokenKind, pos: Position },

    #[error("Invalid statement: {found} at {pos}.")]
    InvalidStmt { found: TokenKind, pos: Position },

    #[error("Invalid type: {found} at {pos}.")]
    InvalidType { found: TokenKind, pos: Position },

    #[error("Invalid declaration: {found} at {pos}.")]
    InvalidDecl { found: TokenKind, pos: Position },

    #[error("Duplicate function: {name} at {pos}.")]
    DuplicateFunction { name: String, pos: Position },

    #[error("Unexpected token: {found} at {pos}, expected {expected}.")]
    UnexpectedToken {
        expected: String,
        found: TokenKind,
        pos: Position,
    },
}

impl ParseError {
    pub fn eof(expected: impl Into<String>) -> Self {
        Self::UnexpectedEOF {
            expected: expected.into(),
        }
    }

    pub fn invalid_expr(found: Token) -> Self {
        Self::InvalidExpr {
            found: found.kind,
            pos: found.pos,
        }
    }

    pub fn invalid_stmt(found: Token) -> Self {
        Self::InvalidStmt {
            found: found.kind,
            pos: found.pos,
        }
    }

    pub fn invalid_type(found: Token) -> Self {
        Self::InvalidType {
            found: found.kind,
            pos: found.pos,
        }
    }

    pub fn invalid_decl(found: Token) -> Self {
        Self::InvalidDecl {
            found: found.kind,
            pos: found.pos,
        }
    }

    pub fn unexpected_token(expected: impl Into<String>, found: Token) -> Self {
        Self::UnexpectedToken {
            expected: expected.into(),
            found: found.kind,
            pos: found.pos,
        }
    }
}
