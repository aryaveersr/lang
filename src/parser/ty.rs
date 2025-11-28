use super::*;

impl Parser<'_> {
    pub(super) fn parse_type(&mut self) -> Type {
        let token = self.expect(To::Identifier, "Missing type expression.");

        Type::Simple {
            name: token.slice.to_owned(),
        }
    }
}
