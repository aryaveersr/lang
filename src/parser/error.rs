use std::num::ParseIntError;

use serde::{Serialize, Serializer};
use thiserror::Error;

use crate::{
    position::Position,
    token::{Token, TokenKind},
};

#[derive(Error, Debug, Clone, PartialEq, Eq, Serialize)]
pub enum ParseError {
    #[error("Missing main function.")]
    MissingMainFunction,

    #[error("Break statement outside of loop at {pos}.")]
    BreakOutsideLoop { pos: Position },

    #[error("Unexpected end of file, expected {expected}.")]
    UnexpectedEOF { expected: String },

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

    #[error("Cannot parse number at {pos}: {err}.")]
    CannotParseNum {
        pos: Position,

        #[serde(serialize_with = "serialize_parse_int_err")]
        #[source]
        err: ParseIntError,
    },
}

impl ParseError {
    pub fn eof<T: Into<String>>(expected: T) -> Self {
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

    pub fn unexpected_token<T: Into<String>>(expected: T, found: Token) -> Self {
        Self::UnexpectedToken {
            expected: expected.into(),
            found: found.kind,
            pos: found.pos,
        }
    }
}

fn serialize_parse_int_err<S: Serializer>(err: &ParseIntError, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&err.to_string())
}
