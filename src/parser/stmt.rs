use super::*;

impl Parser<'_> {
    pub(super) fn parse_body(&mut self, consumed_brace: bool) -> Result<Vec<Stmt>> {
        let mut body = Vec::new();

        if consumed_brace || self.eat(To::LeftBrace).is_some() {
            while self.eat(To::RightBrace).is_none() {
                body.push(self.parse_stmt()?);
            }
        } else {
            body.push(self.parse_stmt()?);
        }

        Ok(body)
    }

    fn parse_stmt_block(&mut self) -> Result<Stmt> {
        Ok(Stmt::Block {
            body: self.parse_body(true)?,
        })
    }

    fn parse_stmt_return(&mut self) -> Result<Stmt> {
        let mut expr = None;

        if self.eat(To::Semicolon).is_none() {
            expr = Some(self.parse_expr()?);
            self.expect(To::Semicolon, Pe::MissingSemicolon)?;
        }

        Ok(Stmt::Return { expr })
    }

    fn parse_stmt_if(&mut self) -> Result<Stmt> {
        self.expect(To::LeftParen, Pe::MissingCondLeftParen)?;

        let cond = self.parse_expr()?;

        self.expect(To::RightParen, Pe::MissingCondRightParen)?;

        let body = self.parse_body(false)?;
        let else_ = self
            .eat(To::Else)
            .map(|_| self.parse_body(false))
            .transpose()?;

        Ok(Stmt::If { cond, body, else_ })
    }

    fn parse_stmt_loop(&mut self) -> Result<Stmt> {
        Ok(Stmt::Loop {
            body: self.parse_body(false)?,
        })
    }

    fn parse_stmt_while(&mut self) -> Result<Stmt> {
        self.expect(To::LeftParen, Pe::MissingCondLeftParen)?;

        let cond = self.parse_expr()?;

        self.expect(To::RightParen, Pe::MissingCondRightParen)?;

        let body = self.parse_body(false)?;

        Ok(Stmt::While { cond, body })
    }

    fn parse_stmt_break(&mut self) -> Result<Stmt> {
        self.expect(To::Semicolon, Pe::MissingSemicolon)?;

        Ok(Stmt::Break)
    }

    fn parse_stmt_let(&mut self) -> Result<Stmt> {
        let token = self.expect(To::Identifier, Pe::MissingVarName)?;
        let ty = self.eat(To::Colon).map(|_| self.parse_type()).transpose()?;
        let expr = self.eat(To::Equal).map(|_| self.parse_expr()).transpose()?;

        self.expect(To::Semicolon, Pe::MissingSemicolon)?;

        Ok(Stmt::Let {
            name: token.slice.to_owned(),
            ty,
            expr,
        })
    }

    pub(super) fn parse_stmt(&mut self) -> Result<Stmt> {
        match self.next()?.kind {
            To::Return => self.parse_stmt_return(),
            To::LeftBrace => self.parse_stmt_block(),
            To::If => self.parse_stmt_if(),
            To::Loop => self.parse_stmt_loop(),
            To::While => self.parse_stmt_while(),
            To::Break => self.parse_stmt_break(),
            To::Let => self.parse_stmt_let(),

            _ => Err(Pe::InvalidStmt),
        }
    }
}
