use crate::{
    hir::{Expr, HirFun, HirModule, HirType, Stmt},
    mir::{BlockID, MirFun, MirModule, MirType, ValueID, builder::Builder},
    scope::Scope,
};

#[derive(Default)]
pub struct HirToMir {
    builder: Builder,
    loop_stack: Vec<BlockID>,
    scope: Scope<ValueID>,
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
        self.builder = Builder::new(name);
        self.builder.fun.return_ty = self.lower_type(&fun.ty.returns);

        self.loop_stack = Vec::new();
        self.scope = Scope::default();

        let entry = self.builder.create_block();
        self.builder.set_active_block(entry);
        self.lower_block(fun.body);

        if self.builder.active_block().term.is_none() {
            self.builder.add_return(None);
        }

        std::mem::take(&mut self.builder).finish()
    }

    fn lower_type(&self, ty: &HirType) -> Option<MirType> {
        match ty {
            HirType::Void => None,
            HirType::Bool => Some(MirType::Bool),
            HirType::Num => Some(MirType::Num),
        }
    }

    fn lower_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Block { body } => self.lower_block(body),

            Stmt::Break => {
                let target = self.loop_stack.pop().unwrap();
                self.builder.add_jump(target);

                let unreachable = self.builder.create_block();
                self.builder.set_active_block(unreachable);
            }

            Stmt::Return { expr } => {
                let value = expr.map(|e| self.lower_expr(*e));
                self.builder.add_return(value);

                let unreachable = self.builder.create_block();
                self.builder.set_active_block(unreachable);
            }

            Stmt::Loop { body } => {
                let body_block = self.builder.create_block();
                let exit_block = self.builder.create_block();

                self.builder.add_jump(body_block);
                self.builder.set_active_block(body_block);
                self.loop_stack.push(exit_block);
                self.lower_block(body);
                self.loop_stack.pop();

                if self.builder.active_block().term.is_none() {
                    self.builder.add_jump(body_block);
                }

                self.builder.set_active_block(exit_block);
            }

            Stmt::If { cond, body, else_ } => {
                let then_block = self.builder.create_block();
                let else_block = self.builder.create_block();
                let exit_block = self.builder.create_block();

                let cond = self.lower_expr(*cond);

                self.builder.add_branch(cond, then_block, else_block);
                self.builder.set_active_block(then_block);
                self.lower_block(body);

                if self.builder.active_block().term.is_none() {
                    self.builder.add_jump(exit_block);
                }

                self.builder.set_active_block(else_block);

                if let Some(else_body) = else_ {
                    self.lower_block(else_body);
                }

                if self.builder.active_block().term.is_none() {
                    self.builder.add_jump(exit_block);
                }

                self.builder.set_active_block(exit_block);
            }

            Stmt::Let { name, ty, expr } => {
                let value = if let Some(expr) = expr {
                    self.lower_expr(*expr)
                } else {
                    match ty.unwrap() {
                        HirType::Bool => self.builder.add_const_bool(false),
                        HirType::Num => self.builder.add_const_num(0),

                        #[expect(clippy::panic)]
                        HirType::Void => panic!("Value cannot be of type void."),
                    }
                };

                self.scope.set(&name, &value);
            }

            Stmt::Assign { name, expr } => {
                let value = self.lower_expr(*expr);
                self.scope.set(&name, &value);
            }

            Stmt::Call { name, args } => {
                self.lower_expr_call(name, args);
            }
        }
    }

    fn lower_block(&mut self, stmts: Vec<Stmt>) {
        self.scope.create();

        for stmt in stmts {
            self.lower_stmt(stmt);
        }

        self.scope.pop();
    }

    fn lower_expr(&mut self, expr: Expr) -> ValueID {
        match expr {
            Expr::Bool { value } => self.builder.add_const_bool(value),
            Expr::Num { value } => self.builder.add_const_num(value),
            Expr::Var { name } => self.scope.get(&name).unwrap().to_owned(),
            Expr::Call { name, args } => self.lower_expr_call(name, args),

            Expr::Unary { op, expr } => {
                let arg = self.lower_expr(*expr);
                self.builder.add_unary(op, arg)
            }

            Expr::Binary { op, lhs, rhs } => {
                let lhs = self.lower_expr(*lhs);
                let rhs = self.lower_expr(*rhs);
                self.builder.add_binary(op, lhs, rhs)
            }
        }
    }

    fn lower_expr_call(&mut self, name: String, args: Vec<Box<Expr>>) -> ValueID {
        let args = args.into_iter().map(|a| self.lower_expr(*a)).collect();
        self.builder.add_call(name, args)
    }
}
