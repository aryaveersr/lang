use super::{Expr, HirFun, HirModule, Stmt};

pub trait Walkable<E> {
    fn walk<V: HirVisitor<E> + ?Sized>(&mut self, visitor: &mut V) -> Result<(), E>;
}

pub trait HirVisitor<E> {
    fn visit_module(&mut self, module: &mut HirModule) -> Result<(), E> {
        module.walk(self)
    }

    fn visit_fun(&mut self, _name: &str, fun: &mut HirFun) -> Result<(), E> {
        fun.walk(self)
    }

    fn visit_stmt(&mut self, stmt: &mut Stmt) -> Result<(), E> {
        stmt.walk(self)
    }

    fn visit_block(&mut self, block: &mut Vec<Stmt>) -> Result<(), E> {
        block.walk(self)
    }

    fn visit_expr(&mut self, expr: &mut Expr) -> Result<(), E> {
        expr.walk(self)
    }
}

impl<E> Walkable<E> for HirModule {
    fn walk<V: HirVisitor<E> + ?Sized>(&mut self, visitor: &mut V) -> Result<(), E> {
        for (name, fun) in &mut self.funs {
            visitor.visit_fun(name, fun)?;
        }

        Ok(())
    }
}

impl<E> Walkable<E> for HirFun {
    fn walk<V: HirVisitor<E> + ?Sized>(&mut self, visitor: &mut V) -> Result<(), E> {
        visitor.visit_block(&mut self.body)
    }
}

impl<E> Walkable<E> for Vec<Stmt> {
    fn walk<V: HirVisitor<E> + ?Sized>(&mut self, visitor: &mut V) -> Result<(), E> {
        for stmt in self {
            visitor.visit_stmt(stmt)?;
        }

        Ok(())
    }
}

impl<E> Walkable<E> for Stmt {
    fn walk<V: HirVisitor<E> + ?Sized>(&mut self, visitor: &mut V) -> Result<(), E> {
        match self {
            Self::Break => {}
            Self::Loop { body } | Self::Block { body } => visitor.visit_block(body)?,

            Self::Return { expr } | Self::Let { expr, .. } => {
                if let Some(expr) = expr {
                    visitor.visit_expr(expr)?;
                }
            }

            Self::If { cond, body, else_ } => {
                visitor.visit_expr(cond)?;
                visitor.visit_block(body)?;

                if let Some(else_block) = else_ {
                    visitor.visit_block(else_block)?;
                }
            }
        }

        Ok(())
    }
}

impl<E> Walkable<E> for Expr {
    fn walk<V: HirVisitor<E> + ?Sized>(&mut self, visitor: &mut V) -> Result<(), E> {
        match self {
            Self::Bool { .. } | Self::Num { .. } | Self::Var { .. } => {}
            Self::Unary { expr, .. } => visitor.visit_expr(expr)?,

            Self::Binary { lhs, rhs, .. } => {
                visitor.visit_expr(lhs)?;
                visitor.visit_expr(rhs)?;
            }
        }

        Ok(())
    }
}
