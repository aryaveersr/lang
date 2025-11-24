use super::*;

impl Parser<'_> {
    fn parse_expr_numeric(&mut self, token: Token) -> Result<Box<Expr>> {
        let value = token.slice.parse().map_err(|_| Pe::NumberTooLarge)?;

        Ok(Box::new(Expr::Num { value }))
    }

    fn parse_expr_group(&mut self) -> Result<Box<Expr>> {
        let expr = self.parse_expr()?;

        self.expect(To::RightParen, Pe::MissingClosingParen)?;

        Ok(expr)
    }

    fn parse_expr_identifier(&mut self, token: Token) -> Result<Box<Expr>> {
        let name = token.slice.to_owned();

        Ok(Box::new(Expr::Var { name }))
    }

    fn parse_expr_primary(&mut self) -> Result<Box<Expr>> {
        let next = self.next()?;

        match next.kind {
            To::Numeric => self.parse_expr_numeric(next),
            To::Identifier => self.parse_expr_identifier(next),
            To::LeftParen => self.parse_expr_group(),
            To::True => Ok(Box::new(Expr::Bool { value: true })),
            To::False => Ok(Box::new(Expr::Bool { value: false })),

            _ => Err(Pe::InvalidExpr),
        }
    }

    fn parse_expr_unary(&mut self) -> Result<Box<Expr>> {
        if let Some(op) = self.map(|kind| match kind {
            To::Minus => Some(UnOp::Negate),
            To::Not => Some(UnOp::Not),
            _ => None,
        }) {
            let expr = self.parse_expr_primary()?;

            Ok(Box::new(Expr::Unary { op, expr }))
        } else {
            self.parse_expr_primary()
        }
    }

    fn parse_expr_factor(&mut self) -> Result<Box<Expr>> {
        let mut lhs = self.parse_expr_unary()?;

        while let Some(op) = self.map(|kind| match kind {
            To::Star => Some(BinOp::Mul),
            To::Slash => Some(BinOp::Div),
            _ => None,
        }) {
            let rhs = self.parse_expr_unary()?;
            lhs = Box::new(Expr::Binary { op, lhs, rhs });
        }

        Ok(lhs)
    }

    fn parse_expr_term(&mut self) -> Result<Box<Expr>> {
        let mut lhs = self.parse_expr_factor()?;

        while let Some(op) = self.map(|kind| match kind {
            To::Plus => Some(BinOp::Add),
            To::Minus => Some(BinOp::Sub),
            _ => None,
        }) {
            let rhs = self.parse_expr_factor()?;
            lhs = Box::new(Expr::Binary { op, lhs, rhs });
        }

        Ok(lhs)
    }

    fn parse_expr_comparison(&mut self) -> Result<Box<Expr>> {
        let mut lhs = self.parse_expr_term()?;

        if let Some(op) = self.map(|kind| match kind {
            To::EqualEqual => Some(BinOp::Eq),
            To::NotEqual => Some(BinOp::NotEq),
            To::Lesser => Some(BinOp::Lesser),
            To::LesserEqual => Some(BinOp::LesserEq),
            To::Greater => Some(BinOp::Greater),
            To::GreaterEqual => Some(BinOp::GreaterEq),
            _ => None,
        }) {
            let rhs = self.parse_expr_term()?;
            lhs = Box::new(Expr::Binary { op, lhs, rhs });
        }

        Ok(lhs)
    }

    fn parse_expr_equality(&mut self) -> Result<Box<Expr>> {
        let mut lhs = self.parse_expr_comparison()?;

        if let Some(op) = self.map(|kind| match kind {
            To::EqualEqual => Some(BinOp::Eq),
            To::NotEqual => Some(BinOp::NotEq),
            _ => None,
        }) {
            let rhs = self.parse_expr_comparison()?;
            lhs = Box::new(Expr::Binary { op, lhs, rhs });
        }

        Ok(lhs)
    }

    fn parse_expr_and(&mut self) -> Result<Box<Expr>> {
        let mut lhs = self.parse_expr_equality()?;

        if self.eat(To::And).is_some() {
            let rhs = self.parse_expr_equality()?;

            lhs = Box::new(Expr::Binary {
                op: BinOp::And,
                lhs,
                rhs,
            });
        }

        Ok(lhs)
    }

    fn parse_expr_or(&mut self) -> Result<Box<Expr>> {
        let mut lhs = self.parse_expr_and()?;

        if self.eat(To::Or).is_some() {
            let rhs = self.parse_expr_and()?;

            lhs = Box::new(Expr::Binary {
                op: BinOp::Or,
                lhs,
                rhs,
            });
        }

        Ok(lhs)
    }

    pub(super) fn parse_expr(&mut self) -> Result<Box<Expr>> {
        self.parse_expr_or()
    }
}
