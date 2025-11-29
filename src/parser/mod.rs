mod expr;
mod stmt;
mod ty;
mod utils;

use crate::{hir::*, lexer::*, ops::*, token::*};
use std::{collections::HashMap, iter::Peekable, panic};

type To = TokenKind;

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

    fn parse_function(&mut self) -> (String, Fun) {
        let name = self.expect(To::Identifier, "Missing function name.");

        self.expect(To::LeftParen, "Missing '('.");
        self.expect(To::RightParen, "Missing ')'.");

        let return_ty = self.eat(To::Colon).map(|_| self.parse_type());

        self.expect(To::LeftBrace, "Missing function body.");

        let body = self.parse_body(true);

        (name.slice.to_owned(), Fun { return_ty, body })
    }

    pub fn parse(&mut self) -> Module {
        let mut funs = HashMap::new();

        while let Some(token) = self.lexer.next() {
            match token.kind {
                To::Fun => {
                    let (name, fun) = self.parse_function();
                    funs.insert(name, fun);
                }

                _ => panic!("Expected EOF or declaration."),
            }
        }

        assert!(funs.contains_key("main"), "No `main()` function found.");

        Module { funs }
    }
}
