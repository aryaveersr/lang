use crate::{
    hir::{Expr, HirFun, HirModule, HirType, Stmt},
    mir::{BlockID, MirFun, MirModule, MirType, Reg, Value},
    mir_builder::MirBuilder,
    scope::Scope,
};

#[derive(Default)]
pub struct HirToMir {
    loop_stack: Vec<BlockID>,
    scope: Scope<Reg>,
}

impl HirToMir {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn lower_module(&mut self, module: HirModule) -> MirModule {
        let mut funs = Vec::new();

        for (name, fun) in module.funs {
            funs.push(self.lower_fun(name, fun));
        }

        MirModule { funs }
    }

    fn lower_fun(&mut self, name: String, fun: HirFun) -> MirFun {
        debug_assert!(self.loop_stack.is_empty());

        let mut builder = MirBuilder::new(name);
        builder.set_return_type(self.lower_type(&fun.ty.returns));

        self.lower_block(&mut builder, fun.body);

        builder.finish()
    }

    fn lower_block(&mut self, builder: &mut MirBuilder, stmts: Vec<Stmt>) {
        self.scope.create();

        for stmt in stmts {
            self.lower_stmt(builder, stmt);
        }

        self.scope.pop();
    }

    fn lower_stmt(&mut self, builder: &mut MirBuilder, stmt: Stmt) {
        match stmt {
            Stmt::Block { body } => self.lower_block(builder, body),

            Stmt::Break => {
                let target = self.loop_stack.last().unwrap();
                builder.build_jump(*target);

                let unreachable = builder.create_block();
                builder.set_active(unreachable);
                builder.seal_block(unreachable);
            }

            Stmt::Return { expr } => {
                let value = expr.map(|e| self.lower_expr(builder, e));
                builder.build_return(value);

                let unreachable = builder.create_block();
                builder.set_active(unreachable);
                builder.seal_block(unreachable);
            }

            Stmt::Loop { body } => {
                let body_block = builder.create_block();
                let exit_block = builder.create_block();

                builder.build_jump(body_block);
                builder.set_active(body_block);

                self.loop_stack.push(exit_block);
                self.lower_block(builder, body);
                self.loop_stack.pop();

                if !builder.is_terminated() {
                    builder.build_jump(body_block);
                }

                builder.seal_block(body_block);
                builder.seal_block(exit_block);
                builder.set_active(exit_block);
            }

            Stmt::If { cond, body, else_ } => {
                let then_block = builder.create_block();
                let else_block = builder.create_block();
                let exit_block = builder.create_block();

                let cond = self.lower_expr(builder, cond);

                builder.build_branch(cond, then_block, else_block);

                builder.seal_block(then_block);
                builder.seal_block(else_block);

                builder.set_active(then_block);

                self.lower_block(builder, body);

                if !builder.is_terminated() {
                    builder.build_jump(exit_block);
                }

                builder.set_active(else_block);

                if let Some(else_body) = else_ {
                    self.lower_block(builder, else_body);
                }

                if !builder.is_terminated() {
                    builder.build_jump(exit_block);
                }

                builder.seal_block(exit_block);
                builder.set_active(exit_block);
            }

            Stmt::Let { name, ty, expr } => {
                let value = if let Some(expr) = expr {
                    self.lower_expr(builder, expr)
                } else {
                    self.lower_type(&ty.unwrap()).unwrap().default_value()
                };

                let value_id = builder.declare_var();
                self.scope.set(name, &value_id);

                builder.assign_var(value_id, value);
            }

            Stmt::Assign { name, expr } => {
                let value = self.lower_expr(builder, expr);
                let reg = self.scope.get(name).unwrap();

                builder.assign_var(*reg, value);
            }

            Stmt::Call { name, args } => {
                self.lower_expr_call(builder, name, args);
            }
        }
    }

    fn lower_expr(&mut self, builder: &mut MirBuilder, expr: Expr) -> Value {
        match expr {
            Expr::Bool { value } => Value::Bool(value),
            Expr::Num { value } => Value::Num(value),
            Expr::Var { name } => Value::Reg(self.scope.get(name).unwrap().to_owned()),
            Expr::Call { name, args } => self.lower_expr_call(builder, name, args),

            Expr::Unary { op, expr } => {
                let arg = self.lower_expr(builder, *expr);

                builder.build_unary(op, arg)
            }

            Expr::Binary { op, lhs, rhs } => {
                let lhs = self.lower_expr(builder, *lhs);
                let rhs = self.lower_expr(builder, *rhs);

                builder.build_binary(op, lhs, rhs)
            }
        }
    }

    fn lower_expr_call(
        &mut self,
        builder: &mut MirBuilder,
        name: String,
        args: Vec<Expr>,
    ) -> Value {
        let args = args
            .into_iter()
            .map(|e| self.lower_expr(builder, e))
            .collect();

        builder.build_call(name, args)
    }

    fn lower_type(&self, ty: &HirType) -> Option<MirType> {
        match ty {
            HirType::Void => None,
            HirType::Bool => Some(MirType::Bool),
            HirType::Num => Some(MirType::Num),
        }
    }
}
