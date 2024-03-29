use wave_ast::ast::{BindingPattern, BindingPatternKind};
use wave_diagnostics::Result;

use crate::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn parse_binding(&mut self) -> Result<(BindingPattern<'a>, bool)> {
        let kind = self.parse_binding_pattern_identifier()?;
        Ok((self.ast.binding_pattern(kind), false))
    }

    fn parse_binding_pattern_identifier(&mut self) -> Result<BindingPatternKind<'a>> {
        self.parse_binding_identifier()
            .map(|ident| self.ast.binding_pattern_identifier(ident))
    }

    pub(crate) fn parse_binding_pattern(&mut self) -> Result<BindingPattern<'a>> {
        let _span = self.start_span();
        let pattern = self.parse_binding()?.0;
        Ok(pattern)
    }
}
