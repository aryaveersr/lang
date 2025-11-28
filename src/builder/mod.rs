mod expr;
mod stmt;
mod ty;

use crate::{ast, hir::*, ops::*, scope::*};
use std::collections::HashMap;

#[derive(Default)]
pub struct Builder {
    scope: Scope<Type>,
    expected_return_type: Option<Type>,
    in_loop: bool,
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    fn lower_function(&mut self, fun: ast::Fun) -> Fun {
        let ty = fun.ty.map(|t| self.lower_ty(t)).unwrap_or(Type::Void);

        self.expected_return_type = Some(ty.clone());

        let body = self.lower_block(fun.body);

        Fun { body, ty }
    }

    pub fn build_hir(&mut self, ast: ast::Ast) -> Module {
        let funs: HashMap<_, _> = ast
            .funs
            .into_iter()
            .map(|f| (f.name.clone(), self.lower_function(f)))
            .collect();

        assert!(funs.contains_key("main"), "No `main()` function found.");

        Module { funs }
    }
}
