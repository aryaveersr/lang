use std::{collections::HashMap, iter::Peekable};

use crate::{
    hir::{HirFun, HirFunType, HirModule, HirType},
    lexer::Lexer,
    parser::error::ParseError,
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

    fn parse_params(&mut self) -> Result<Vec<(String, HirType)>> {
        let mut params = Vec::new();

        self.expect(TokenKind::LeftParen, "(")?;

        if self.eat(TokenKind::RightParen).is_some() {
            return Ok(params);
        }

        loop {
            let name = self.expect(TokenKind::Identifier, "parameter name")?.slice;
            self.expect(TokenKind::Colon, "parameter type")?;
            let ty = self.parse_type()?;

            params.push((name.to_owned(), ty));

            if self.eat(TokenKind::Comma).is_none() {
                break;
            }
        }

        self.expect(TokenKind::RightParen, ")")?;

        Ok(params)
    }

    fn parse_function(&mut self) -> Result<(String, HirFun)> {
        let name = self.expect(TokenKind::Identifier, "function name")?;
        let params = self.parse_params()?;

        let returns = self
            .eat(TokenKind::Colon)
            .map_or(Ok(HirType::Void), |_| self.parse_type())?;

        self.expect(TokenKind::LeftBrace, "function body")?;

        let body = self.parse_body(true)?;

        Ok((
            name.slice.to_owned(),
            HirFun {
                body,
                ty: HirFunType { params, returns },
            },
        ))
    }

    pub fn parse(&mut self) -> Result<HirModule> {
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
            Ok(HirModule { funs })
        } else {
            Err(ParseError::MissingMainFunction)
        }
    }
}
