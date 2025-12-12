use crate::{
    hir::Expr,
    ops::{BinOp, UnOp},
};

impl Expr {
    pub fn bool(value: bool) -> Expr {
        Expr::Bool { value }
    }

    pub fn unary(op: UnOp, expr: Expr) -> Expr {
        Expr::Unary {
            op,
            expr: Box::new(expr),
        }
    }

    pub fn binary(op: BinOp, lhs: Expr, rhs: Expr) -> Expr {
        Expr::Binary {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
}
