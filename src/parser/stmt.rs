use super::*;

impl Parser<'_> {
    pub(super) fn parse_stmt(&mut self) -> Stmt {
        match self.next().kind {
            To::Return => self.parse_stmt_return(),
            To::LeftBrace => self.parse_stmt_block(),
            To::If => self.parse_stmt_if(),
            To::Loop => self.parse_stmt_loop(),
            To::While => self.parse_stmt_while(),
            To::Break => self.parse_stmt_break(),
            To::Let => self.parse_stmt_let(),

            _ => panic!("Invalid statement."),
        }
    }

    pub(super) fn parse_body(&mut self, until_brace: bool) -> Vec<Stmt> {
        let mut body = Vec::new();

        if until_brace || self.eat(To::LeftBrace).is_some() {
            while self.eat(To::RightBrace).is_none() {
                body.push(self.parse_stmt());
            }
        } else {
            body.push(self.parse_stmt());
        }

        body
    }

    fn parse_condition(&mut self) -> Box<Expr> {
        self.expect(To::LeftParen, "Missing '('.");
        let cond = self.parse_expr();
        self.expect(To::RightParen, "Missing ')'.");

        cond
    }

    fn parse_loop_body(&mut self) -> Vec<Stmt> {
        let old_in_loop = self.in_loop;
        self.in_loop = true;

        let body = self.parse_body(false);

        self.in_loop = old_in_loop;
        body
    }

    fn parse_stmt_return(&mut self) -> Stmt {
        let mut expr = None;

        if self.eat(To::Semicolon).is_none() {
            expr = Some(self.parse_expr());
            self.expect(To::Semicolon, "Missing semicolon.");
        }

        Stmt::Return { expr }
    }

    fn parse_stmt_block(&mut self) -> Stmt {
        let body = self.parse_body(true);

        Stmt::Block { body }
    }

    fn parse_stmt_if(&mut self) -> Stmt {
        let cond = self.parse_condition();
        let body = self.parse_body(false);
        let else_ = self.eat(To::Else).map(|_| self.parse_body(false));

        Stmt::If { cond, body, else_ }
    }

    fn parse_stmt_loop(&mut self) -> Stmt {
        let body = self.parse_loop_body();

        Stmt::Loop { body }
    }

    fn parse_stmt_while(&mut self) -> Stmt {
        let expr = self.parse_condition();
        let mut body = self.parse_loop_body();

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

        Stmt::Loop { body }
    }

    fn parse_stmt_break(&mut self) -> Stmt {
        self.expect(To::Semicolon, "Missing semicolon.");

        if !self.in_loop {
            panic!("Cannot use `break` outside a loop.");
        }

        Stmt::Break
    }

    fn parse_stmt_let(&mut self) -> Stmt {
        let token = self.expect(To::Identifier, "Missing variable name.");
        let name = token.slice.to_owned();
        let ty = self.eat(To::Colon).map(|_| self.parse_type());
        let expr = self.eat(To::Equal).map(|_| self.parse_expr());

        self.expect(To::Semicolon, "Missing semicolon.");

        Stmt::Let { name, ty, expr }
    }
}
