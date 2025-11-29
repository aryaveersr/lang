use super::*;

pub trait Walkable {
    fn walk<V: HirVisitor + ?Sized>(&mut self, visitor: &mut V);
}

pub trait HirVisitor {
    fn visit_module(&mut self, module: &mut Module) {
        module.walk(self);
    }

    fn visit_fun(&mut self, _name: &str, fun: &mut Fun) {
        fun.walk(self);
    }

    fn visit_stmt(&mut self, stmt: &mut Stmt) {
        stmt.walk(self);
    }

    fn visit_block(&mut self, block: &mut Vec<Stmt>) {
        block.walk(self);
    }

    fn visit_expr(&mut self, expr: &mut Expr) {
        expr.walk(self);
    }
}

impl Walkable for Module {
    fn walk<V: HirVisitor + ?Sized>(&mut self, visitor: &mut V) {
        for (name, fun) in &mut self.funs {
            visitor.visit_fun(name, fun);
        }
    }
}

impl Walkable for Fun {
    fn walk<V: HirVisitor + ?Sized>(&mut self, visitor: &mut V) {
        visitor.visit_block(&mut self.body);
    }
}

impl Walkable for Vec<Stmt> {
    fn walk<V: HirVisitor + ?Sized>(&mut self, visitor: &mut V) {
        for stmt in self {
            visitor.visit_stmt(stmt);
        }
    }
}

impl Walkable for Stmt {
    fn walk<V: HirVisitor + ?Sized>(&mut self, visitor: &mut V) {
        match self {
            Self::Break => {}
            Self::Loop { body } | Self::Block { body } => visitor.visit_block(body),

            Self::Return { expr } | Self::Let { expr, .. } => {
                if let Some(expr) = expr {
                    visitor.visit_expr(expr);
                }
            }

            Self::If { cond, body, else_ } => {
                visitor.visit_expr(cond);
                visitor.visit_block(body);

                if let Some(else_block) = else_ {
                    visitor.visit_block(else_block);
                }
            }
        }
    }
}

impl Walkable for Expr {
    fn walk<V: HirVisitor + ?Sized>(&mut self, visitor: &mut V) {
        match self {
            Self::Bool { .. } | Self::Num { .. } | Self::Var { .. } => {}
            Self::Unary { expr, .. } => visitor.visit_expr(expr),

            Self::Binary { lhs, rhs, .. } => {
                visitor.visit_expr(lhs);
                visitor.visit_expr(rhs);
            }
        }
    }
}
