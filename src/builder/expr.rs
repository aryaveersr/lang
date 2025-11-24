use super::*;

impl Builder {
    fn type_unary(&self, op: UnOp, expr: &Expr) -> Result<Type> {
        match op {
            UnOp::Negate => {
                if expr.ty == Type::Num {
                    Ok(Type::Num)
                } else {
                    Err(Be::ExpectedNum)
                }
            }

            UnOp::Not => {
                if expr.ty == Type::Bool {
                    Ok(Type::Bool)
                } else {
                    Err(Be::ExpectedBool)
                }
            }
        }
    }

    fn type_binary(&self, op: BinOp, lhs: &Expr, rhs: &Expr) -> Result<Type> {
        if lhs.ty != rhs.ty {
            return Err(Be::OperandsNotSameType);
        }

        match op {
            BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div => {
                if lhs.ty == Type::Num {
                    Ok(Type::Num)
                } else {
                    Err(Be::ExpectedNum)
                }
            }

            BinOp::Lesser | BinOp::LesserEq | BinOp::Greater | BinOp::GreaterEq => {
                if lhs.ty == Type::Num {
                    Ok(Type::Bool)
                } else {
                    Err(Be::ExpectedNum)
                }
            }

            BinOp::And | BinOp::Or => {
                if lhs.ty == Type::Bool {
                    Ok(Type::Bool)
                } else {
                    Err(Be::ExpectedBool)
                }
            }

            BinOp::Eq | BinOp::NotEq => Ok(Type::Bool),
        }
    }

    fn lower_expr_unary(&mut self, op: UnOp, expr: ast::Expr) -> Result<Expr> {
        let expr = self.lower_expr(expr)?;
        let ty = self.type_unary(op, &expr)?;

        Ok(Expr {
            ty,
            kind: ExprKind::Unary { op, expr },
        })
    }

    fn lower_expr_binary(&mut self, op: BinOp, lhs: ast::Expr, rhs: ast::Expr) -> Result<Expr> {
        let lhs = self.lower_expr(lhs)?;
        let rhs = self.lower_expr(rhs)?;
        let ty = self.type_binary(op, &lhs, &rhs)?;

        Ok(Expr {
            ty,
            kind: ExprKind::Binary { op, lhs, rhs },
        })
    }

    pub(super) fn lower_expr(&mut self, expr: ast::Expr) -> Result<Box<Expr>> {
        let expr = match expr {
            ast::Expr::Unary { op, expr } => self.lower_expr_unary(op, *expr)?,
            ast::Expr::Binary { op, lhs, rhs } => self.lower_expr_binary(op, *lhs, *rhs)?,

            ast::Expr::Num { value } => Expr {
                ty: Type::Num,
                kind: ExprKind::Num { value },
            },

            ast::Expr::Bool { value } => Expr {
                ty: Type::Bool,
                kind: ExprKind::Bool { value },
            },

            ast::Expr::Var { name } => Expr {
                ty: self.scope.get(&name).ok_or(Be::VarNotFound)?,
                kind: ExprKind::Var { name },
            },
        };

        Ok(Box::new(expr))
    }
}
