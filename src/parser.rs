mod expr;
mod stmt;
mod ty;
mod utils;

use crate::{
    errors::{ParseError, ParseResult},
    hir::*,
    lexer::*,
    ops::*,
    token::*,
};
use std::{collections::HashMap, iter::Peekable};

type Result<T> = ParseResult<T>;

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    in_loop: bool,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self {
            lexer: lexer.peekable(),
            in_loop: false,
        }
    }

    fn parse_function(&mut self) -> Result<(String, Fun)> {
        let name = self.expect(TokenKind::Identifier, "function name")?;

        self.expect(TokenKind::LeftParen, "(")?;
        self.expect(TokenKind::RightParen, ")")?;

        let return_ty = self
            .eat(TokenKind::Colon)
            .map(|_| self.parse_type())
            .transpose()?;

        self.expect(TokenKind::LeftBrace, "function body")?;

        let body = self.parse_body(true)?;

        Ok((name.slice.to_owned(), Fun { return_ty, body }))
    }

    pub fn parse(&mut self) -> Result<Module> {
        let mut funs = HashMap::new();

        while let Some(token) = self.lexer.next() {
            match token.kind {
                TokenKind::Fun => {
                    let (name, fun) = self.parse_function()?;
                    funs.insert(name, fun);
                }

                _ => return Err(ParseError::eof("declaration")),
            }
        }

        if funs.contains_key("main") {
            Ok(Module { funs })
        } else {
            Err(ParseError::MissingMainFunction)
        }
    }
}
