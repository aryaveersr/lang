use super::*;

impl Parser<'_> {
    pub(super) fn parse_type(&mut self) -> Type {
        let token = self.expect(To::Identifier, "Expected type name.");

        match token.slice {
            "bool" => Type::Bool,
            "num" => Type::Num,
            "void" => Type::Void,

            _ => panic!("Unknown type: {}", token.slice),
        }
    }
}
