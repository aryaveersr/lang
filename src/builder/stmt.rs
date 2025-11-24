use super::*;
use std::collections::HashMap;

impl Builder {
    pub(super) fn lower_block(&mut self, stmts: Vec<ast::Stmt>) -> Result<Block> {
        self.scope.create();

        let stmts = stmts
            .into_iter()
            .map(|s| self.lower_stmt(s))
            .collect::<Result<_>>()?;

        let scope = self.scope.pop();

        Ok(Block { scope, stmts })
    }

    fn lower_stmt_block(&mut self, body: Vec<ast::Stmt>) -> Result<Stmt> {
        Ok(Stmt::Block(self.lower_block(body)?))
    }

    fn lower_stmt_break(&mut self) -> Result<Stmt> {
        if self.in_loop {
            Ok(Stmt::Break)
        } else {
            Err(Be::BreakOutsideLoop)
        }
    }

    fn lower_stmt_loop(&mut self, body: Vec<ast::Stmt>) -> Result<Stmt> {
        self.in_loop = true;
        let body = self.lower_block(body)?;
        self.in_loop = false;

        Ok(Stmt::Loop { body })
    }

    fn lower_stmt_return(&mut self, expr: Option<Box<ast::Expr>>) -> Result<Stmt> {
        let expr = expr.map(|e| self.lower_expr(*e)).transpose()?;
        let expected = self.expected_return_type.as_ref();

        if !match &expr {
            Some(expr) => expected == Some(&expr.ty),
            None => expected == Some(&Type::Void),
        } {
            return Err(Be::WrongReturnType);
        }

        Ok(Stmt::Return { expr })
    }

    fn lower_stmt_while(&mut self, cond: ast::Expr, body: Vec<ast::Stmt>) -> Result<Stmt> {
        let cond = self.lower_expr(cond)?;

        if cond.ty != Type::Bool {
            return Err(Be::ExpectedBool);
        }

        self.in_loop = true;
        let mut body = self.lower_block(body)?;
        self.in_loop = false;

        let if_stmt = Stmt::If {
            cond: Box::new(Expr {
                ty: Type::Bool,
                kind: ExprKind::Unary {
                    op: UnOp::Not,
                    expr: cond,
                },
            }),
            body: Block {
                scope: HashMap::new(),
                stmts: vec![Stmt::Break],
            },
            else_: None,
        };

        body.stmts.insert(0, if_stmt);

        Ok(Stmt::Loop { body })
    }

    fn lower_stmt_if(
        &mut self,
        cond: ast::Expr,
        body: Vec<ast::Stmt>,
        else_: Option<Vec<ast::Stmt>>,
    ) -> Result<Stmt> {
        let cond = self.lower_expr(cond)?;
        let body = self.lower_block(body)?;
        let else_ = else_.map(|s| self.lower_block(s)).transpose()?;

        if cond.ty == Type::Bool {
            Ok(Stmt::If { cond, body, else_ })
        } else {
            Err(Be::ExpectedBool)
        }
    }

    fn lower_stmt_let(
        &mut self,
        name: String,
        ty: Option<ast::Type>,
        expr: Option<Box<ast::Expr>>,
    ) -> Result<Stmt> {
        let expr = expr.map(|e| self.lower_expr(*e)).transpose()?;
        let value_ty = expr.as_ref().map(|e| e.ty.clone());

        let ty = match (self.lower_ty_opt(ty)?, value_ty) {
            (Some(ty), None) | (None, Some(ty)) => ty,
            (Some(a), Some(b)) if a == b => a,

            (None, None) => return Err(Be::CannotInferType),
            _ => return Err(Be::TypeMismatch),
        };

        self.scope.set(&name, &ty);

        Ok(Stmt::Let { name, expr, ty })
    }

    pub(super) fn lower_stmt(&mut self, stmt: ast::Stmt) -> Result<Stmt> {
        match stmt {
            ast::Stmt::Break => self.lower_stmt_break(),
            ast::Stmt::Loop { body } => self.lower_stmt_loop(body),
            ast::Stmt::Return { expr } => self.lower_stmt_return(expr),
            ast::Stmt::Block { body } => self.lower_stmt_block(body),
            ast::Stmt::While { cond, body } => self.lower_stmt_while(*cond, body),
            ast::Stmt::Let { name, ty, expr } => self.lower_stmt_let(name, ty, expr),
            ast::Stmt::If { cond, body, else_ } => self.lower_stmt_if(*cond, body, else_),
        }
    }
}
