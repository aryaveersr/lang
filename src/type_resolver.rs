use self::error::TypeError;
use crate::{
    hir::{
        Expr, Fun, Module, Stmt, Type,
        visitor::{HirVisitor, Walkable as _},
    },
    ops::{BinOp, UnOp},
    scope::Scope,
};

pub mod error;

type Result<T> = std::result::Result<T, TypeError>;

#[derive(Default)]
pub struct TypeResolver {
    scope: Scope<Type>,
    expected_return_type: Option<Type>,
}

impl HirVisitor<TypeError> for TypeResolver {
    fn visit_fun(&mut self, _name: &str, fun: &mut Fun) -> Result<()> {
        self.expected_return_type = Some(fun.return_ty.get_or_insert(Type::Void).clone());
        self.visit_block(&mut fun.body)?;
        self.expected_return_type = None;

        Ok(())
    }

    fn visit_stmt(&mut self, stmt: &mut Stmt) -> Result<()> {
        #[inline]
        #[expect(clippy::ref_option)]
        fn unbox<T>(x: &Option<Box<T>>) -> Option<&T> {
            x.as_ref().map(|y| &**y)
        }

        match stmt {
            Stmt::Break => Ok(()),
            Stmt::Block { body } | Stmt::Loop { body } => self.visit_block(body),

            Stmt::Return { expr } => self.resolve_stmt_return(unbox(expr)),
            Stmt::Let { name, ty, expr } => self.resolve_stmt_let(name, ty, unbox(expr)),
            Stmt::If { cond, body, else_ } => self.resolve_stmt_if(cond, body, else_),
        }
    }

    fn visit_block(&mut self, block: &mut Vec<Stmt>) -> Result<()> {
        self.scope.create();
        block.walk(self)?;
        self.scope.pop();

        Ok(())
    }

    fn visit_expr(&mut self, expr: &mut Expr) -> Result<()> {
        self.resolve_expr(expr)?;
        Ok(())
    }
}

impl TypeResolver {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn resolve(&mut self, module: &mut Module) -> Result<()> {
        self.visit_module(module)
    }

    fn resolve_expr(&self, expr: &Expr) -> Result<Type> {
        match expr {
            Expr::Bool { .. } => Ok(Type::Bool),
            Expr::Num { .. } => Ok(Type::Num),
            Expr::Unary { op, expr } => self.resolve_expr_unary(*op, expr),
            Expr::Binary { op, lhs, rhs } => self.resolve_expr_binary(*op, lhs, rhs),

            Expr::Var { name } => {
                self.scope
                    .get(name)
                    .cloned()
                    .ok_or_else(|| TypeError::UndefinedVar {
                        name: name.to_owned(),
                    })
            }
        }
    }

    fn resolve_expr_unary(&self, op: UnOp, expr: &Expr) -> Result<Type> {
        let ty = self.resolve_expr(expr)?;

        match (op, &ty) {
            (UnOp::Negate, Type::Num) => Ok(Type::Num),
            (UnOp::Not, Type::Bool) => Ok(Type::Bool),

            _ => Err(TypeError::InvalidUnaryOp { op, ty }),
        }
    }

    fn resolve_expr_binary(&self, op: BinOp, lhs: &Expr, rhs: &Expr) -> Result<Type> {
        let lhs = self.resolve_expr(lhs)?;
        let rhs = self.resolve_expr(rhs)?;

        if lhs != rhs {
            return Err(TypeError::InvalidBinaryOp { op, lhs, rhs });
        }

        match (op, &lhs) {
            (BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div, Type::Num) => Ok(Type::Num),

            (BinOp::Eq | BinOp::NotEq, _)
            | (BinOp::And | BinOp::Or, Type::Bool)
            | (BinOp::Lesser | BinOp::LesserEq | BinOp::Greater | BinOp::GreaterEq, Type::Num) => {
                Ok(Type::Bool)
            }

            _ => Err(TypeError::InvalidBinaryOp { op, lhs, rhs }),
        }
    }

    fn resolve_stmt_let(
        &mut self,
        name: &str,
        ty: &mut Option<Type>,
        expr: Option<&Expr>,
    ) -> Result<()> {
        let expr_ty = expr.as_ref().map(|e| self.resolve_expr(e)).transpose()?;

        let resolved_ty = match (ty, expr_ty) {
            (Some(annotated_ty), None) => annotated_ty.clone(),

            (ty @ None, Some(inferred_ty)) => {
                *ty = Some(inferred_ty.clone());
                inferred_ty
            }

            (Some(annotated_ty), Some(inferred_ty)) => {
                let annotated_ty = annotated_ty.clone();

                if annotated_ty == inferred_ty {
                    annotated_ty
                } else {
                    return Err(TypeError::TypeMismatch {
                        expected: annotated_ty,
                        found: inferred_ty,
                    });
                }
            }

            (None, None) => {
                return Err(TypeError::CannotInferType {
                    name: name.to_owned(),
                });
            }
        };

        self.scope.set(name, &resolved_ty);
        Ok(())
    }

    fn resolve_stmt_return(&self, expr: Option<&Expr>) -> Result<()> {
        let fun_ty = self.expected_return_type.as_ref().unwrap();
        let expr_ty = expr
            .as_ref()
            .map(|e| self.resolve_expr(e))
            .transpose()?
            .unwrap_or(Type::Void);

        if expr_ty == *fun_ty {
            Ok(())
        } else {
            Err(TypeError::TypeMismatch {
                expected: fun_ty.clone(),
                found: expr_ty,
            })
        }
    }

    fn resolve_stmt_if(
        &mut self,
        cond: &Expr,
        body: &mut Vec<Stmt>,
        else_: &mut Option<Vec<Stmt>>,
    ) -> Result<()> {
        let cond_ty = self.resolve_expr(cond)?;

        if cond_ty != Type::Bool {
            return Err(TypeError::NonBooleanCondition { found: cond_ty });
        }

        self.visit_block(body)?;
        if let Some(block) = else_.as_mut() {
            self.visit_block(block)?;
        }

        Ok(())
    }
}
