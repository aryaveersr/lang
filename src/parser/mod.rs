mod errors;
mod expr;
mod stmt;
mod ty;
mod utils;

pub use errors::ParseError;

use crate::{ast::*, lexer::*, ops::*, token::*};
use errors::*;
use std::{iter::Peekable, panic};

type To = TokenKind;
type Pe = ParseError;

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self {
            lexer: lexer.peekable(),
        }
    }

    fn parse_function(&mut self) -> Result<Function> {
        let name = self.expect(To::Identifier, Pe::MissingFunName)?;

        self.expect(To::LeftParen, Pe::MissingFunLeftParen)?;
        self.expect(To::RightParen, Pe::MissingFunRightParen)?;

        let ty = self.eat(To::Colon).map(|_| self.parse_type()).transpose()?;

        self.expect(To::LeftBrace, Pe::MissingFunBody)?;

        let body = self.parse_body(true)?;

        Ok(Function {
            name: name.slice.to_owned(),
            ty,
            body,
        })
    }

    pub fn parse(&mut self) -> Result<Ast> {
        let mut functions = Vec::new();

        while let Ok(token) = self.next() {
            match token.kind {
                To::Fun => functions.push(self.parse_function()?),
                _ => panic!("Expected EOF or declaration."),
            }
        }

        Ok(Ast { functions })
    }
}
