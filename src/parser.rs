use std::{collections::HashMap, iter::Peekable};

use self::error::ParseError;
use crate::{
    hir::{Fun, Module},
    lexer::Lexer,
    token::TokenKind,
};

pub mod error;
mod expr;
mod stmt;
mod ty;
mod utils;

type Result<T> = std::result::Result<T, ParseError>;

pub struct Parser<'src> {
    lexer: Peekable<Lexer<'src>>,
    in_loop: bool,
}

impl<'src> Parser<'src> {
    pub fn new(lexer: Lexer<'src>) -> Self {
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

                    if funs.contains_key(&name) {
                        return Err(ParseError::DuplicateFunction {
                            name,
                            pos: token.pos,
                        });
                    }

                    funs.insert(name, fun);
                }

                _ => return Err(ParseError::invalid_decl(token)),
            }
        }

        if funs.contains_key("main") {
            Ok(Module { funs })
        } else {
            Err(ParseError::MissingMainFunction)
        }
    }
}
