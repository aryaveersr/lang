use super::*;

impl Builder {
    pub(super) fn lower_ty(&mut self, ty: ast::Type) -> Result<Type> {
        match ty {
            ast::Type::Simple { name } => match name.as_str() {
                "void" => Ok(Type::Void),
                "bool" => Ok(Type::Bool),
                "num" => Ok(Type::Num),

                _ => Err(Be::TypeNotFound),
            },
        }
    }

    pub(super) fn lower_ty_opt(&mut self, ty: Option<ast::Type>) -> Result<Option<Type>> {
        ty.map(|ty| self.lower_ty(ty)).transpose()
    }
}
