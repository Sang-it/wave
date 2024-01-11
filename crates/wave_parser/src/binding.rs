use wave_ast::ast::{BindingPattern, BindingPatternKind};
use wave_diagnostics::Result;

use crate::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn parse_binding(&mut self) -> Result<(BindingPattern<'a>, bool)> {
        let kind = match self.cur_kind() {
            _ => self.parse_binding_pattern_identifier(),
        }?;
        Ok((self.ast.binding_pattern(kind), false))
    }

    fn parse_binding_pattern_identifier(&mut self) -> Result<BindingPatternKind<'a>> {
        self.parse_binding_identifier()
            .map(|ident| self.ast.binding_pattern_identifier(ident))
    }
}
