use super::*;

impl Builder {
    pub(super) fn lower_ty(&mut self, ty: ast::Type) -> Type {
        match ty {
            ast::Type::Simple { name } => match name.as_str() {
                "void" => Type::Void,
                "bool" => Type::Bool,
                "num" => Type::Num,

                _ => panic!("Invalid type name: {name}."),
            },
        }
    }
}
