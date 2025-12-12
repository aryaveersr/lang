use crate::hir::{Expr, Stmt};

impl Stmt {
    pub fn if_(cond: Expr, body: Vec<Self>, else_: Option<Vec<Self>>) -> Self {
        Self::If { cond, body, else_ }
    }

    pub fn assign(name: String, expr: Expr) -> Self {
        Self::Assign { name, expr }
    }

    pub fn call(name: String, args: Vec<Expr>) -> Self {
        Self::Call { name, args }
    }
}
