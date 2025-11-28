use super::*;

impl Parser<'_> {
    fn parse_expr_numeric(&mut self, token: Token) -> Box<Expr> {
        let value = token.slice.parse().expect("Number too large.");

        Box::new(Expr::Num { value })
    }

    fn parse_expr_group(&mut self) -> Box<Expr> {
        let expr = self.parse_expr();

        self.expect(To::RightParen, "Missing ')'.");

        expr
    }

    fn parse_expr_identifier(&mut self, token: Token) -> Box<Expr> {
        let name = token.slice.to_owned();

        Box::new(Expr::Var { name })
    }

    fn parse_expr_primary(&mut self) -> Box<Expr> {
        let next = self.next();

        match next.kind {
            To::Numeric => self.parse_expr_numeric(next),
            To::Identifier => self.parse_expr_identifier(next),
            To::LeftParen => self.parse_expr_group(),
            To::True => Box::new(Expr::Bool { value: true }),
            To::False => Box::new(Expr::Bool { value: false }),

            _ => panic!("Invalid expression: {next:?}."),
        }
    }

    fn parse_expr_unary(&mut self) -> Box<Expr> {
        if let Some(op) = self.map(|kind| match kind {
            To::Minus => Some(UnOp::Negate),
            To::Not => Some(UnOp::Not),
            _ => None,
        }) {
            let expr = self.parse_expr_primary();

            Box::new(Expr::Unary { op, expr })
        } else {
            self.parse_expr_primary()
        }
    }

    fn parse_expr_factor(&mut self) -> Box<Expr> {
        let mut lhs = self.parse_expr_unary();

        while let Some(op) = self.map(|kind| match kind {
            To::Star => Some(BinOp::Mul),
            To::Slash => Some(BinOp::Div),
            _ => None,
        }) {
            let rhs = self.parse_expr_unary();
            lhs = Box::new(Expr::Binary { op, lhs, rhs });
        }

        lhs
    }

    fn parse_expr_term(&mut self) -> Box<Expr> {
        let mut lhs = self.parse_expr_factor();

        while let Some(op) = self.map(|kind| match kind {
            To::Plus => Some(BinOp::Add),
            To::Minus => Some(BinOp::Sub),
            _ => None,
        }) {
            let rhs = self.parse_expr_factor();
            lhs = Box::new(Expr::Binary { op, lhs, rhs });
        }

        lhs
    }

    fn parse_expr_comparison(&mut self) -> Box<Expr> {
        let mut lhs = self.parse_expr_term();

        if let Some(op) = self.map(|kind| match kind {
            To::EqualEqual => Some(BinOp::Eq),
            To::NotEqual => Some(BinOp::NotEq),
            To::Lesser => Some(BinOp::Lesser),
            To::LesserEqual => Some(BinOp::LesserEq),
            To::Greater => Some(BinOp::Greater),
            To::GreaterEqual => Some(BinOp::GreaterEq),
            _ => None,
        }) {
            let rhs = self.parse_expr_term();
            lhs = Box::new(Expr::Binary { op, lhs, rhs });
        }

        lhs
    }

    fn parse_expr_equality(&mut self) -> Box<Expr> {
        let mut lhs = self.parse_expr_comparison();

        if let Some(op) = self.map(|kind| match kind {
            To::EqualEqual => Some(BinOp::Eq),
            To::NotEqual => Some(BinOp::NotEq),
            _ => None,
        }) {
            let rhs = self.parse_expr_comparison();
            lhs = Box::new(Expr::Binary { op, lhs, rhs });
        }

        lhs
    }

    fn parse_expr_and(&mut self) -> Box<Expr> {
        let mut lhs = self.parse_expr_equality();

        if self.eat(To::And).is_some() {
            let rhs = self.parse_expr_equality();

            lhs = Box::new(Expr::Binary {
                op: BinOp::And,
                lhs,
                rhs,
            });
        }

        lhs
    }

    fn parse_expr_or(&mut self) -> Box<Expr> {
        let mut lhs = self.parse_expr_and();

        if self.eat(To::Or).is_some() {
            let rhs = self.parse_expr_and();

            lhs = Box::new(Expr::Binary {
                op: BinOp::Or,
                lhs,
                rhs,
            });
        }

        lhs
    }

    pub(super) fn parse_expr(&mut self) -> Box<Expr> {
        self.parse_expr_or()
    }
}
