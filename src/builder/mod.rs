mod errors;
mod expr;
mod scope;
mod stmt;
mod ty;

pub use errors::BuilderError;

use crate::{ast, hir::*, ops::*};
use errors::*;
use scope::*;
use std::collections::HashMap;

type Be = BuilderError;

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

    fn lower_function(&mut self, fun: ast::Fun) -> Result<Fun> {
        let ty = self.lower_ty_opt(fun.ty)?.unwrap_or(Type::Void);

        self.expected_return_type = Some(ty.clone());

        let body = self.lower_block(fun.body)?;

        Ok(Fun { body, ty })
    }

    pub fn build_hir(&mut self, ast: ast::Ast) -> Result<Module> {
        let funs: HashMap<_, _> = ast
            .funs
            .into_iter()
            .map(|f| Ok((f.name.clone(), self.lower_function(f)?)))
            .collect::<Result<_>>()?;

        if funs.contains_key("main") {
            Ok(Module { funs })
        } else {
            Err(Be::MainNotFound)
        }
    }
}
