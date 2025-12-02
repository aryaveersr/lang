use super::{ParseError, Parser, Result};
use crate::{
    hir::Expr,
    ops::{BinOp, UnOp},
    token::TokenKind,
};

impl Parser<'_> {
    pub(super) fn parse_expr(&mut self) -> Result<Box<Expr>> {
        self.parse_expr_or()
    }

    fn parse_expr_or(&mut self) -> Result<Box<Expr>> {
        let mut lhs = self.parse_expr_and()?;

        while self.eat(TokenKind::Or).is_some() {
            let rhs = self.parse_expr_and()?;
            lhs = Box::new(Expr::Binary {
                op: BinOp::Or,
                lhs,
                rhs,
            });
        }

        Ok(lhs)
    }

    fn parse_expr_and(&mut self) -> Result<Box<Expr>> {
        let mut lhs = self.parse_expr_eq()?;

        while self.eat(TokenKind::And).is_some() {
            let rhs = self.parse_expr_eq()?;
            lhs = Box::new(Expr::Binary {
                op: BinOp::And,
                lhs,
                rhs,
            });
        }

        Ok(lhs)
    }

    fn parse_expr_eq(&mut self) -> Result<Box<Expr>> {
        let mut lhs = self.parse_expr_cmp()?;

        while let Some(op) = self.eat_map(|kind| match kind {
            TokenKind::EqualEqual => Some(BinOp::Eq),
            TokenKind::NotEqual => Some(BinOp::NotEq),
            _ => None,
        }) {
            let rhs = self.parse_expr_cmp()?;
            lhs = Box::new(Expr::Binary { op, lhs, rhs });
        }

        Ok(lhs)
    }

    fn parse_expr_cmp(&mut self) -> Result<Box<Expr>> {
        let mut lhs = self.parse_expr_term()?;

        while let Some(op) = self.eat_map(|kind| match kind {
            TokenKind::Lesser => Some(BinOp::Lesser),
            TokenKind::LesserEqual => Some(BinOp::LesserEq),
            TokenKind::Greater => Some(BinOp::Greater),
            TokenKind::GreaterEqual => Some(BinOp::GreaterEq),
            _ => None,
        }) {
            let rhs = self.parse_expr_term()?;
            lhs = Box::new(Expr::Binary { op, lhs, rhs });
        }

        Ok(lhs)
    }

    fn parse_expr_term(&mut self) -> Result<Box<Expr>> {
        let mut lhs = self.parse_expr_factor()?;

        while let Some(op) = self.eat_map(|kind| match kind {
            TokenKind::Plus => Some(BinOp::Add),
            TokenKind::Minus => Some(BinOp::Sub),
            _ => None,
        }) {
            let rhs = self.parse_expr_factor()?;
            lhs = Box::new(Expr::Binary { op, lhs, rhs });
        }

        Ok(lhs)
    }

    fn parse_expr_factor(&mut self) -> Result<Box<Expr>> {
        let mut lhs = self.parse_expr_unary()?;

        while let Some(op) = self.eat_map(|kind| match kind {
            TokenKind::Star => Some(BinOp::Mul),
            TokenKind::Slash => Some(BinOp::Div),
            _ => None,
        }) {
            let rhs = self.parse_expr_unary()?;
            lhs = Box::new(Expr::Binary { op, lhs, rhs });
        }

        Ok(lhs)
    }

    fn parse_expr_unary(&mut self) -> Result<Box<Expr>> {
        if let Some(op) = self.eat_map(|kind| match kind {
            TokenKind::Minus => Some(UnOp::Negate),
            TokenKind::Not => Some(UnOp::Not),
            _ => None,
        }) {
            let expr = self.parse_expr_unary()?;
            Ok(Box::new(Expr::Unary { op, expr }))
        } else {
            self.parse_expr_primary()
        }
    }

    fn parse_expr_primary(&mut self) -> Result<Box<Expr>> {
        let next = self.next("expression")?;

        Ok(match next.kind {
            TokenKind::True => Box::new(Expr::Bool { value: true }),
            TokenKind::False => Box::new(Expr::Bool { value: false }),

            TokenKind::Identifier => {
                let name = next.slice.to_owned();

                Box::new(Expr::Var { name })
            }

            TokenKind::Numeric => {
                let value = next
                    .slice
                    .parse()
                    .map_err(|err| ParseError::CannotParseNum { pos: next.pos, err })?;

                Box::new(Expr::Num { value })
            }

            TokenKind::LeftParen => {
                let expr = self.parse_expr()?;
                self.expect(TokenKind::RightParen, ")")?;

                expr
            }

            _ => return Err(ParseError::invalid_expr(next)),
        })
    }
}
