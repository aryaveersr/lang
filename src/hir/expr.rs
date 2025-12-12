use crate::{
    hir::Expr,
    ops::{BinOp, UnOp},
};

impl Expr {
    pub fn bool(value: bool) -> Self {
        Self::Bool { value }
    }

    pub fn unary(op: UnOp, expr: Self) -> Self {
        Self::Unary {
            op,
            expr: Box::new(expr),
        }
    }

    pub fn binary(op: BinOp, lhs: Self, rhs: Self) -> Self {
        Self::Binary {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
}
