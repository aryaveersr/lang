use super::*;
use std::collections::HashMap;

impl Builder {
    pub(super) fn lower_block(&mut self, stmts: Vec<ast::Stmt>) -> Block {
        self.scope.create();

        let stmts = stmts.into_iter().map(|s| self.lower_stmt(s)).collect();
        let scope = self.scope.pop();

        Block { scope, stmts }
    }

    fn lower_stmt_block(&mut self, body: Vec<ast::Stmt>) -> Stmt {
        Stmt::Block(self.lower_block(body))
    }

    fn lower_stmt_break(&mut self) -> Stmt {
        assert!(self.in_loop, "Cannot use `break` outside a loop.");
        Stmt::Break
    }

    fn lower_stmt_loop(&mut self, body: Vec<ast::Stmt>) -> Stmt {
        self.in_loop = true;
        let body = self.lower_block(body);
        self.in_loop = false;

        Stmt::Loop { body }
    }

    fn lower_stmt_return(&mut self, expr: Option<Box<ast::Expr>>) -> Stmt {
        let expr = expr.map(|e| self.lower_expr(*e));
        let expected = self.expected_return_type.as_ref();

        if !match &expr {
            Some(expr) => expected == Some(&expr.ty),
            None => expected == Some(&Type::Void),
        } {
            panic!("Expression returned doesn't match function's return type.");
        }

        Stmt::Return { expr }
    }

    fn lower_stmt_while(&mut self, cond: ast::Expr, body: Vec<ast::Stmt>) -> Stmt {
        let cond = self.lower_expr(cond);

        assert_eq!(
            cond.ty,
            Type::Bool,
            "Expected condition expression to be a boolean."
        );

        self.in_loop = true;
        let mut body = self.lower_block(body);
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

        Stmt::Loop { body }
    }

    fn lower_stmt_if(
        &mut self,
        cond: ast::Expr,
        body: Vec<ast::Stmt>,
        else_: Option<Vec<ast::Stmt>>,
    ) -> Stmt {
        let cond = self.lower_expr(cond);
        let body = self.lower_block(body);
        let else_ = else_.map(|s| self.lower_block(s));

        assert_eq!(
            cond.ty,
            Type::Bool,
            "Expected condition expression to be a boolean."
        );

        Stmt::If { cond, body, else_ }
    }

    fn lower_stmt_let(
        &mut self,
        name: String,
        ty: Option<ast::Type>,
        expr: Option<Box<ast::Expr>>,
    ) -> Stmt {
        let expr = expr.map(|e| self.lower_expr(*e));
        let value_ty = expr.as_ref().map(|e| e.ty.clone());

        let ty = match (ty.map(|t| self.lower_ty(t)), value_ty) {
            (Some(ty), None) | (None, Some(ty)) => ty,
            (Some(a), Some(b)) if a == b => a,

            (None, None) => panic!("Cannot infer type in assignment."),
            _ => panic!("Type annotation in assignment doesn't match the expression."),
        };

        self.scope.set(&name, &ty);

        Stmt::Let { name, expr, ty }
    }

    pub(super) fn lower_stmt(&mut self, stmt: ast::Stmt) -> Stmt {
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
