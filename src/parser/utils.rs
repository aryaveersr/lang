use super::{ParseError, Parser, Result};
use crate::token::{Token, TokenKind};

impl<'src> Parser<'src> {
    pub(super) fn eat(&mut self, kind: TokenKind) -> Option<Token<'src>> {
        self.lexer.next_if(|i| i.kind == kind)
    }

    pub(super) fn eat_map<T>(&mut self, f: impl Fn(TokenKind) -> Option<T>) -> Option<T> {
        self.lexer
            .next_if(|token| f(token.kind).is_some())
            .map(|t| t.kind)
            .and_then(f)
    }

    pub(super) fn next(&mut self, expected: impl Into<String>) -> Result<Token<'src>> {
        self.lexer.next().ok_or_else(|| ParseError::eof(expected))
    }

    pub(super) fn expect(&mut self, kind: TokenKind, expected: &str) -> Result<Token<'src>> {
        let next = self.lexer.next().ok_or_else(|| ParseError::eof(expected))?;

        if next.kind == kind {
            Ok(next)
        } else {
            Err(ParseError::unexpected_token(expected, next))
        }
    }
}
