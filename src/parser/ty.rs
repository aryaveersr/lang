use crate::{
    hir::HirType,
    parser::{ParseError, Parser, Result},
    token::TokenKind,
};

impl Parser<'_> {
    pub(super) fn parse_type(&mut self) -> Result<HirType> {
        let token = self.expect(TokenKind::Identifier, "type name")?;

        Ok(match token.slice {
            "bool" => HirType::Bool,
            "num" => HirType::Num,
            "void" => HirType::Void,

            _ => return Err(ParseError::invalid_type(token)),
        })
    }
}
