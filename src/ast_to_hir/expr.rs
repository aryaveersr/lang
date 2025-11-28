use super::*;

impl AstToHir {
    fn type_unary(&self, op: UnOp, expr: &Expr) -> Type {
        match op {
            UnOp::Negate => {
                assert_eq!(expr.ty, Type::Num, "Expected number operand in negation.");
                Type::Num
            }

            UnOp::Not => {
                assert_eq!(expr.ty, Type::Bool, "Expected boolean operand in not.");
                Type::Bool
            }
        }
    }

    fn type_binary(&self, op: BinOp, lhs: &Expr, rhs: &Expr) -> Type {
        assert_eq!(
            lhs.ty, rhs.ty,
            "Cannot perform binary operation on expressions of different type."
        );

        match op {
            BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div => {
                assert_eq!(lhs.ty, Type::Num, "Expected number operands for binary op");
                Type::Num
            }

            BinOp::Lesser | BinOp::LesserEq | BinOp::Greater | BinOp::GreaterEq => {
                assert_eq!(lhs.ty, Type::Num, "Expected number operands for binary op");
                Type::Bool
            }

            BinOp::And | BinOp::Or => {
                assert_eq!(lhs.ty, Type::Bool, "Expected bool operands for binary op");
                Type::Bool
            }

            BinOp::Eq | BinOp::NotEq => Type::Bool,
        }
    }

    fn lower_expr_unary(&mut self, op: UnOp, expr: ast::Expr) -> Expr {
        let expr = self.lower_expr(expr);
        let ty = self.type_unary(op, &expr);

        Expr {
            ty,
            kind: ExprKind::Unary { op, expr },
        }
    }

    fn lower_expr_binary(&mut self, op: BinOp, lhs: ast::Expr, rhs: ast::Expr) -> Expr {
        let lhs = self.lower_expr(lhs);
        let rhs = self.lower_expr(rhs);
        let ty = self.type_binary(op, &lhs, &rhs);

        Expr {
            ty,
            kind: ExprKind::Binary { op, lhs, rhs },
        }
    }

    pub(super) fn lower_expr(&mut self, expr: ast::Expr) -> Box<Expr> {
        let expr = match expr {
            ast::Expr::Unary { op, expr } => self.lower_expr_unary(op, *expr),
            ast::Expr::Binary { op, lhs, rhs } => self.lower_expr_binary(op, *lhs, *rhs),

            ast::Expr::Num { value } => Expr {
                ty: Type::Num,
                kind: ExprKind::Num { value },
            },

            ast::Expr::Bool { value } => Expr {
                ty: Type::Bool,
                kind: ExprKind::Bool { value },
            },

            ast::Expr::Var { name } => Expr {
                ty: self.scope.get(&name).expect("Variable not found in scope."),
                kind: ExprKind::Var { name },
            },
        };

        Box::new(expr)
    }
}
