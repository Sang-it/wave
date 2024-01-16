use crate::Parser;
use wave_ast::ast::PropertyKey;
use wave_diagnostics::Result;
use wave_lexer::Kind;

impl<'a> Parser<'a> {
    pub(crate) fn parse_property_name(&mut self) -> Result<PropertyKey<'a>> {
        let key = match self.cur_kind() {
            Kind::Str => self
                .parse_literal_expression()
                .map(PropertyKey::Expression)?,
            kind if kind.is_number() => self
                .parse_literal_expression()
                .map(PropertyKey::Expression)?,
            _ => {
                let ident = self.parse_identifier_name()?;
                PropertyKey::Identifier(self.ast.alloc(ident))
            }
        };
        Ok(key)
    }
}
