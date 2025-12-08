use std::collections::HashMap;

use crate::{
    hir::{Expr, HirFun, HirFunType, HirModule, HirType, Stmt},
    ops::{BinOp, UnOp},
    scope::Scope,
    type_resolver::error::TypeError,
};

pub mod error;

type Result<T> = std::result::Result<T, TypeError>;

#[derive(Default)]
pub struct TypeResolver {
    scope: Scope<HirType>,
    functions: HashMap<String, HirFunType>,
    expected_return_type: Option<HirType>,
}

impl TypeResolver {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn resolve(&mut self, module: &mut HirModule) -> Result<()> {
        for (name, fun) in &mut module.funs {
            self.functions.insert(name.to_owned(), fun.ty.clone());
        }

        for fun in module.funs.values_mut() {
            self.resolve_fun(fun)?;
        }

        Ok(())
    }

    fn resolve_fun(&mut self, fun: &mut HirFun) -> Result<()> {
        self.expected_return_type = Some(fun.ty.returns.clone());
        self.scope.create();

        for (name, ty) in &fun.ty.params {
            self.scope.set(name, ty);
        }

        self.resolve_block(&mut fun.body)?;

        self.scope.pop();
        self.expected_return_type = None;

        Ok(())
    }

    fn resolve_stmt(&mut self, stmt: &mut Stmt) -> Result<()> {
        match stmt {
            Stmt::Break => Ok(()),
            Stmt::Block { body } | Stmt::Loop { body } => self.resolve_block(body),
            Stmt::Return { expr } => self.resolve_stmt_return(expr.as_ref()),
            Stmt::Let { name, ty, expr } => self.resolve_stmt_let(name, ty, expr.as_ref()),
            Stmt::If { cond, body, else_ } => self.resolve_stmt_if(cond, body, else_),
            Stmt::Assign { name, expr } => self.resolve_stmt_assign(name, expr),
            Stmt::Call { name, args } => self.resolve_expr_call(name, args).map(|_| ()),
        }
    }

    fn resolve_block(&mut self, block: &mut Vec<Stmt>) -> Result<()> {
        self.scope.create();

        for stmt in block {
            self.resolve_stmt(stmt)?;
        }

        self.scope.pop();

        Ok(())
    }

    fn resolve_expr(&self, expr: &Expr) -> Result<HirType> {
        match expr {
            Expr::Bool { .. } => Ok(HirType::Bool),
            Expr::Num { .. } => Ok(HirType::Num),
            Expr::Unary { op, expr } => self.resolve_expr_unary(*op, expr),
            Expr::Binary { op, lhs, rhs } => self.resolve_expr_binary(*op, lhs, rhs),
            Expr::Call { name, args } => self.resolve_expr_call(name, args),

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

    fn resolve_expr_unary(&self, op: UnOp, expr: &Expr) -> Result<HirType> {
        let ty = self.resolve_expr(expr)?;

        match (op, &ty) {
            (UnOp::Negate, HirType::Num) => Ok(HirType::Num),
            (UnOp::Not, HirType::Bool) => Ok(HirType::Bool),

            _ => Err(TypeError::InvalidUnaryOp { op, ty }),
        }
    }

    fn resolve_expr_binary(&self, op: BinOp, lhs: &Expr, rhs: &Expr) -> Result<HirType> {
        let lhs = self.resolve_expr(lhs)?;
        let rhs = self.resolve_expr(rhs)?;

        if lhs != rhs {
            return Err(TypeError::InvalidBinaryOp { op, lhs, rhs });
        }

        match (op, &lhs) {
            (BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div, HirType::Num) => Ok(HirType::Num),

            (BinOp::Eq | BinOp::NotEq, _)
            | (BinOp::And | BinOp::Or, HirType::Bool)
            | (BinOp::Lesser | BinOp::LesserEq | BinOp::Greater | BinOp::GreaterEq, HirType::Num) => {
                Ok(HirType::Bool)
            }

            _ => Err(TypeError::InvalidBinaryOp { op, lhs, rhs }),
        }
    }

    fn resolve_expr_call(&self, name: &str, args: &[Expr]) -> Result<HirType> {
        let ty = self
            .functions
            .get(name)
            .cloned()
            .ok_or_else(|| TypeError::UndefinedFun {
                name: name.to_owned(),
            })?;

        if args.len() != ty.params.len() {
            return Err(TypeError::InvalidCallArgs {
                name: name.to_owned(),
                expected: ty.params.len(),
                found: args.len(),
            });
        }

        for (arg, param) in args.iter().zip(ty.params.iter()) {
            let arg_ty = self.resolve_expr(arg)?;

            if arg_ty != param.1 {
                return Err(TypeError::TypeMismatch {
                    expected: param.1.clone(),
                    found: arg_ty,
                });
            }
        }

        Ok(ty.returns)
    }

    fn resolve_stmt_let(
        &mut self,
        name: &str,
        ty: &mut Option<HirType>,
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
            .unwrap_or(HirType::Void);

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

        if cond_ty != HirType::Bool {
            return Err(TypeError::NonBooleanCondition { found: cond_ty });
        }

        self.resolve_block(body)?;
        if let Some(block) = else_.as_mut() {
            self.resolve_block(block)?;
        }

        Ok(())
    }

    fn resolve_stmt_assign(&self, name: &str, expr: &Expr) -> Result<()> {
        let expr_ty = self.resolve_expr(expr)?;
        let var_ty = self
            .scope
            .get(name)
            .ok_or_else(|| TypeError::UndefinedVar {
                name: name.to_owned(),
            })?;

        if expr_ty == *var_ty {
            Ok(())
        } else {
            Err(TypeError::TypeMismatch {
                expected: var_ty.clone(),
                found: expr_ty,
            })
        }
    }
}
