use crate::{
    hir::Expr,
    ops::{BinOp, UnOp},
    parser::{ParseError, Parser, Result},
    token::{Token, TokenKind},
};

impl Parser<'_> {
    pub(super) fn parse_expr(&mut self) -> Result<Expr> {
        self.parse_expr_or()
    }

    fn parse_expr_or(&mut self) -> Result<Expr> {
        let mut lhs = self.parse_expr_and()?;

        while self.eat(TokenKind::Or).is_some() {
            let rhs = self.parse_expr_and()?;
            lhs = Expr::Binary {
                op: BinOp::Or,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
        }

        Ok(lhs)
    }

    fn parse_expr_and(&mut self) -> Result<Expr> {
        let mut lhs = self.parse_expr_eq()?;

        while self.eat(TokenKind::And).is_some() {
            let rhs = self.parse_expr_eq()?;
            lhs = Expr::Binary {
                op: BinOp::And,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
        }

        Ok(lhs)
    }

    fn parse_expr_eq(&mut self) -> Result<Expr> {
        let mut lhs = self.parse_expr_cmp()?;

        while let Some(op) = self.eat_map(|kind| match kind {
            TokenKind::EqualEqual => Some(BinOp::Eq),
            TokenKind::NotEqual => Some(BinOp::NotEq),
            _ => None,
        }) {
            let rhs = self.parse_expr_cmp()?;
            lhs = Expr::Binary {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
        }

        Ok(lhs)
    }

    fn parse_expr_cmp(&mut self) -> Result<Expr> {
        let mut lhs = self.parse_expr_term()?;

        while let Some(op) = self.eat_map(|kind| match kind {
            TokenKind::Lesser => Some(BinOp::Lesser),
            TokenKind::LesserEqual => Some(BinOp::LesserEq),
            TokenKind::Greater => Some(BinOp::Greater),
            TokenKind::GreaterEqual => Some(BinOp::GreaterEq),
            _ => None,
        }) {
            let rhs = self.parse_expr_term()?;
            lhs = Expr::Binary {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
        }

        Ok(lhs)
    }

    fn parse_expr_term(&mut self) -> Result<Expr> {
        let mut lhs = self.parse_expr_factor()?;

        while let Some(op) = self.eat_map(|kind| match kind {
            TokenKind::Plus => Some(BinOp::Add),
            TokenKind::Minus => Some(BinOp::Sub),
            _ => None,
        }) {
            let rhs = self.parse_expr_factor()?;
            lhs = Expr::Binary {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
        }

        Ok(lhs)
    }

    fn parse_expr_factor(&mut self) -> Result<Expr> {
        let mut lhs = self.parse_expr_unary()?;

        while let Some(op) = self.eat_map(|kind| match kind {
            TokenKind::Star => Some(BinOp::Mul),
            TokenKind::Slash => Some(BinOp::Div),
            _ => None,
        }) {
            let rhs = self.parse_expr_unary()?;
            lhs = Expr::Binary {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
        }

        Ok(lhs)
    }

    fn parse_expr_unary(&mut self) -> Result<Expr> {
        if let Some(op) = self.eat_map(|kind| match kind {
            TokenKind::Minus => Some(UnOp::Negate),
            TokenKind::Not => Some(UnOp::Not),
            _ => None,
        }) {
            let expr = self.parse_expr_unary()?;
            Ok(Expr::Unary {
                op,
                expr: Box::new(expr),
            })
        } else {
            self.parse_expr_primary()
        }
    }

    fn parse_expr_primary(&mut self) -> Result<Expr> {
        let next = self.next("expression")?;

        Ok(match next.kind {
            TokenKind::Numeric => self.parse_expr_numeric(next)?,
            TokenKind::Identifier => self.parse_expr_identifier(next)?,
            TokenKind::LeftParen => self.parse_expr_group()?,
            TokenKind::True => Expr::Bool { value: true },
            TokenKind::False => Expr::Bool { value: false },

            _ => return Err(ParseError::invalid_expr(next)),
        })
    }

    fn parse_expr_numeric(&self, token: Token) -> Result<Expr> {
        let value = token
            .slice
            .parse()
            .map_err(|err| ParseError::CannotParseNum {
                pos: token.pos,
                err,
            })?;

        Ok(Expr::Num { value })
    }

    fn parse_expr_group(&mut self) -> Result<Expr> {
        let expr = self.parse_expr()?;
        self.expect(TokenKind::RightParen, ")")?;

        Ok(expr)
    }

    fn parse_expr_identifier(&mut self, token: Token) -> Result<Expr> {
        let name = token.slice.to_owned();

        let expr = if self.eat(TokenKind::LeftParen).is_some() {
            let args = self.parse_args()?;

            Expr::Call { name, args }
        } else {
            Expr::Var { name }
        };

        Ok(expr)
    }
}
