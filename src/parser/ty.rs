use super::*;

impl Parser<'_> {
    pub(super) fn parse_type(&mut self) -> Result<Type> {
        let token = self.expect(TokenKind::Identifier, "type name")?;

        Ok(match token.slice {
            "bool" => Type::Bool,
            "num" => Type::Num,
            "void" => Type::Void,

            _ => return Err(ParseError::invalid_type(token)),
        })
    }
}
