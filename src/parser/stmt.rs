use crate::{
    hir::{Expr, Stmt},
    ops::UnOp,
    parser::{ParseError, Parser, Result},
    token::{Token, TokenKind},
};

impl Parser<'_> {
    pub(super) fn parse_stmt(&mut self) -> Result<Stmt> {
        let next = self.next("statement")?;

        match next.kind {
            TokenKind::Return => self.parse_stmt_return(),
            TokenKind::LeftBrace => self.parse_stmt_block(),
            TokenKind::If => self.parse_stmt_if(),
            TokenKind::Loop => self.parse_stmt_loop(),
            TokenKind::While => self.parse_stmt_while(),
            TokenKind::Break => self.parse_stmt_break(next),
            TokenKind::Let => self.parse_stmt_let(),
            TokenKind::Identifier => self.parse_stmt_identifier(next),

            _ => Err(ParseError::invalid_stmt(next)),
        }
    }

    pub(super) fn parse_body(&mut self, until_brace: bool) -> Result<Vec<Stmt>> {
        let mut body = Vec::new();

        if until_brace || self.eat(TokenKind::LeftBrace).is_some() {
            while self.eat(TokenKind::RightBrace).is_none() {
                body.push(self.parse_stmt()?);
            }
        } else {
            body.push(self.parse_stmt()?);
        }

        Ok(body)
    }

    fn parse_stmt_return(&mut self) -> Result<Stmt> {
        let mut expr = None;

        if self.eat(TokenKind::Semicolon).is_none() {
            expr = Some(self.parse_expr()?);
            self.expect(TokenKind::Semicolon, ";")?;
        }

        Ok(Stmt::Return { expr })
    }

    fn parse_stmt_block(&mut self) -> Result<Stmt> {
        let body = self.parse_body(true)?;

        Ok(Stmt::Block { body })
    }

    fn parse_stmt_if(&mut self) -> Result<Stmt> {
        let cond = self.parse_condition()?;
        let body = self.parse_body(false)?;

        let else_ = self
            .eat(TokenKind::Else)
            .map(|_| self.parse_body(false))
            .transpose()?;

        Ok(Stmt::If { cond, body, else_ })
    }

    fn parse_stmt_loop(&mut self) -> Result<Stmt> {
        let body = self.parse_loop_body()?;

        Ok(Stmt::Loop { body })
    }

    fn parse_stmt_while(&mut self) -> Result<Stmt> {
        let expr = self.parse_condition()?;
        let mut body = self.parse_loop_body()?;

        body.insert(
            0,
            Stmt::If {
                body: vec![Stmt::Break],
                else_: None,
                cond: Box::new(Expr::Unary {
                    op: UnOp::Not,
                    expr,
                }),
            },
        );

        Ok(Stmt::Loop { body })
    }

    fn parse_stmt_break(&mut self, token: Token) -> Result<Stmt> {
        self.expect(TokenKind::Semicolon, ";")?;

        if self.in_loop {
            Ok(Stmt::Break)
        } else {
            Err(ParseError::BreakOutsideLoop { pos: token.pos })
        }
    }

    fn parse_stmt_let(&mut self) -> Result<Stmt> {
        let token = self.expect(TokenKind::Identifier, "variable name")?;
        let name = token.slice.to_owned();

        let ty = self
            .eat(TokenKind::Colon)
            .map(|_| self.parse_type())
            .transpose()?;

        let expr = self
            .eat(TokenKind::Equal)
            .map(|_| self.parse_expr())
            .transpose()?;

        self.expect(TokenKind::Semicolon, ";")?;

        Ok(Stmt::Let { name, ty, expr })
    }

    fn parse_stmt_identifier(&mut self, token: Token) -> Result<Stmt> {
        todo!()
    }

    fn parse_condition(&mut self) -> Result<Box<Expr>> {
        self.expect(TokenKind::LeftParen, "(")?;
        let cond = self.parse_expr()?;
        self.expect(TokenKind::RightParen, ")")?;

        Ok(cond)
    }

    fn parse_loop_body(&mut self) -> Result<Vec<Stmt>> {
        let old_in_loop = self.in_loop;
        self.in_loop = true;

        let body = self.parse_body(false)?;

        self.in_loop = old_in_loop;
        Ok(body)
    }
}
