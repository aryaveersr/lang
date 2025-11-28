mod expr;
mod stmt;
mod ty;
mod utils;

use crate::{ast::*, lexer::*, ops::*, token::*};
use std::{iter::Peekable, panic};

type To = TokenKind;

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self {
            lexer: lexer.peekable(),
        }
    }

    fn parse_function(&mut self) -> Fun {
        let name = self.expect(To::Identifier, "Missing function name.");

        self.expect(To::LeftParen, "Missing '('.");
        self.expect(To::RightParen, "Missing ')'.");

        let ty = self.eat(To::Colon).map(|_| self.parse_type());

        self.expect(To::LeftBrace, "Missing function body.");

        let body = self.parse_body(true);

        Fun {
            name: name.slice.to_owned(),
            ty,
            body,
        }
    }

    pub fn parse(&mut self) -> Ast {
        let mut funs = Vec::new();

        while let Some(token) = self.lexer.next() {
            match token.kind {
                To::Fun => funs.push(self.parse_function()),
                _ => panic!("Expected EOF or declaration."),
            }
        }

        Ast { funs }
    }
}
