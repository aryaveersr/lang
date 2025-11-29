use super::*;

#[derive(Default)]
pub struct TypeResolver {
    scope: Scope<Type>,
    expected_return_type: Option<Type>,
}

impl TypeResolver {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn resolve(&mut self, module: &mut Module) {
        self.visit_module(module);
    }

    fn resolve_expr(&mut self, expr: &mut Expr) -> Type {
        match expr {
            Expr::Bool { .. } => Type::Bool,
            Expr::Num { .. } => Type::Num,
            Expr::Unary { op, expr } => self.resolve_expr_unary(*op, expr),
            Expr::Binary { op, lhs, rhs } => self.resolve_expr_binary(*op, lhs, rhs),

            Expr::Var { name } => match self.scope.get(name) {
                Some(ty) => ty.clone(),
                None => {
                    panic!("Variable '{}' not found in scope", name);
                }
            },
        }
    }

    fn resolve_expr_unary(&mut self, op: UnOp, expr: &mut Expr) -> Type {
        let expr_ty = self.resolve_expr(expr);

        match op {
            UnOp::Negate => {
                if expr_ty != Type::Num {
                    panic!("Expected number operand for negation, found {:?}", expr_ty);
                }

                Type::Num
            }
            UnOp::Not => {
                if expr_ty != Type::Bool {
                    panic!(
                        "Expected boolean operand for logical not, found {:?}",
                        expr_ty
                    );
                }

                Type::Bool
            }
        }
    }

    fn resolve_expr_binary(&mut self, op: BinOp, lhs: &mut Expr, rhs: &mut Expr) -> Type {
        let lhs_ty = self.resolve_expr(lhs);
        let rhs_ty = self.resolve_expr(rhs);

        if lhs_ty != rhs_ty {
            panic!(
                "Type mismatch in binary operation: {:?} vs {:?}",
                lhs_ty, rhs_ty
            );
        }

        match op {
            BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div => {
                if lhs_ty != Type::Num {
                    panic!(
                        "Expected numeric operands for arithmetic operation, found {:?}",
                        lhs_ty
                    );
                }

                Type::Num
            }

            BinOp::Lesser | BinOp::LesserEq | BinOp::Greater | BinOp::GreaterEq => {
                if lhs_ty != Type::Num {
                    panic!(
                        "Expected numeric operands for comparison, found {:?}",
                        lhs_ty
                    );
                }

                Type::Bool
            }

            BinOp::And | BinOp::Or => {
                if lhs_ty != Type::Bool {
                    panic!(
                        "Expected boolean operands for logical operation, found {:?}",
                        lhs_ty
                    );
                }

                Type::Bool
            }

            BinOp::Eq | BinOp::NotEq => Type::Bool,
        }
    }

    fn resolve_stmt_let(
        &mut self,
        name: &str,
        ty: &mut Option<Type>,
        expr: &mut Option<Box<Expr>>,
    ) {
        let expr_ty = expr.as_mut().map(|e| self.resolve_expr(e));

        let resolved_ty = match (ty, expr_ty) {
            (Some(annotated_ty), None) => annotated_ty.clone(),

            (ty @ None, Some(inferred_ty)) => {
                *ty = Some(inferred_ty.clone());
                inferred_ty
            }

            (Some(annotated_ty), Some(inferred_ty)) => {
                if *annotated_ty != inferred_ty {
                    panic!(
                        "Type annotation {:?} doesn't match expression type {:?} for variable '{}'",
                        annotated_ty, inferred_ty, name
                    );
                }

                annotated_ty.clone()
            }

            (None, None) => {
                panic!(
                    "Cannot infer type for variable '{}' without type annotation or initializer",
                    name
                );
            }
        };

        self.scope.set(name, &resolved_ty);
    }

    fn resolve_stmt_return(&mut self, expr: &mut Option<Box<Expr>>) {
        let return_ty = expr.as_mut().map_or(Type::Void, |e| self.resolve_expr(e));
        let expected = self.expected_return_type.as_ref().unwrap();

        if return_ty != *expected {
            panic!(
                "Return type doesn't match function return type {:?}",
                expected
            );
        }
    }

    fn resolve_stmt_if(
        &mut self,
        cond: &mut Expr,
        body: &mut Vec<Stmt>,
        else_: &mut Option<Vec<Stmt>>,
    ) {
        let cond_ty = self.resolve_expr(cond);
        if cond_ty != Type::Bool {
            panic!(
                "Expected boolean condition in if statement, found {:?}",
                cond_ty
            );
        }

        self.visit_block(body);
        else_.as_mut().map(|block| self.visit_block(block));
    }

    fn resolve_stmt_loop(&mut self, body: &mut Vec<Stmt>) {
        self.visit_block(body);
    }
}

impl HirVisitor for TypeResolver {
    fn visit_fun(&mut self, _name: &str, fun: &mut Fun) {
        self.expected_return_type = Some(fun.return_ty.get_or_insert(Type::Void).clone());
        self.visit_block(&mut fun.body);
        self.expected_return_type = None;
    }

    fn visit_stmt(&mut self, stmt: &mut Stmt) {
        match stmt {
            Stmt::Break => {}
            Stmt::Block { body } => self.visit_block(body),

            Stmt::Return { expr } => self.resolve_stmt_return(expr),
            Stmt::Let { name, ty, expr } => self.resolve_stmt_let(name, ty, expr),
            Stmt::If { cond, body, else_ } => self.resolve_stmt_if(cond, body, else_),
            Stmt::Loop { body } => self.resolve_stmt_loop(body),
        }
    }

    fn visit_block(&mut self, block: &mut Vec<Stmt>) {
        self.scope.create();
        block.walk(self);
        self.scope.pop();
    }

    fn visit_expr(&mut self, expr: &mut Expr) {
        self.resolve_expr(expr);
    }
}
