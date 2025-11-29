use super::*;

impl Parser<'_> {
    pub(super) fn parse_expr(&mut self) -> Box<Expr> {
        self.parse_expr_or()
    }

    fn parse_expr_or(&mut self) -> Box<Expr> {
        let mut lhs = self.parse_expr_and();

        while self.eat(To::Or).is_some() {
            let rhs = self.parse_expr_and();
            lhs = Box::new(Expr::Binary {
                op: BinOp::Or,
                lhs,
                rhs,
            });
        }

        lhs
    }

    fn parse_expr_and(&mut self) -> Box<Expr> {
        let mut lhs = self.parse_expr_eq();

        while self.eat(To::And).is_some() {
            let rhs = self.parse_expr_eq();
            lhs = Box::new(Expr::Binary {
                op: BinOp::And,
                lhs,
                rhs,
            });
        }

        lhs
    }

    fn parse_expr_eq(&mut self) -> Box<Expr> {
        let mut lhs = self.parse_expr_cmp();

        while let Some(op) = self.eat_map(|kind| match kind {
            To::EqualEqual => Some(BinOp::Eq),
            To::NotEqual => Some(BinOp::NotEq),
            _ => None,
        }) {
            let rhs = self.parse_expr_cmp();
            lhs = Box::new(Expr::Binary { op, lhs, rhs });
        }

        lhs
    }

    fn parse_expr_cmp(&mut self) -> Box<Expr> {
        let mut lhs = self.parse_expr_term();

        while let Some(op) = self.eat_map(|kind| match kind {
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

    fn parse_expr_term(&mut self) -> Box<Expr> {
        let mut lhs = self.parse_expr_factor();

        while let Some(op) = self.eat_map(|kind| match kind {
            To::Plus => Some(BinOp::Add),
            To::Minus => Some(BinOp::Sub),
            _ => None,
        }) {
            let rhs = self.parse_expr_factor();
            lhs = Box::new(Expr::Binary { op, lhs, rhs });
        }

        lhs
    }

    fn parse_expr_factor(&mut self) -> Box<Expr> {
        let mut lhs = self.parse_expr_unary();

        while let Some(op) = self.eat_map(|kind| match kind {
            To::Star => Some(BinOp::Mul),
            To::Slash => Some(BinOp::Div),
            _ => None,
        }) {
            let rhs = self.parse_expr_unary();
            lhs = Box::new(Expr::Binary { op, lhs, rhs });
        }

        lhs
    }

    fn parse_expr_unary(&mut self) -> Box<Expr> {
        if let Some(op) = self.eat_map(|kind| match kind {
            To::Minus => Some(UnOp::Negate),
            To::Not => Some(UnOp::Not),
            _ => None,
        }) {
            let expr = self.parse_expr_unary();
            return Box::new(Expr::Unary { op, expr });
        }

        self.parse_expr_primary()
    }

    fn parse_expr_primary(&mut self) -> Box<Expr> {
        let token = self.next();

        match token.kind {
            To::True => Box::new(Expr::Bool { value: true }),
            To::False => Box::new(Expr::Bool { value: false }),

            To::Identifier => Box::new(Expr::Var {
                name: token.slice.into(),
            }),

            To::Numeric => {
                let value = token
                    .slice
                    .parse()
                    .expect("Failed to parse numeric literal");

                Box::new(Expr::Num { value })
            }

            To::LeftParen => {
                let expr = self.parse_expr();
                self.expect(To::RightParen, "Missing closing parenthesis.");
                expr
            }

            _ => panic!("Unexpected token in expression: {:?}", token),
        }
    }
}
