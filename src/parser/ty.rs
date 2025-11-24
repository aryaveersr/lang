use super::*;

impl Parser<'_> {
    pub(super) fn parse_type(&mut self) -> Result<Type> {
        let token = self.expect(To::Identifier, Pe::MissingTypeName)?;

        Ok(Type::Simple {
            name: token.slice.to_owned(),
        })
    }
}
