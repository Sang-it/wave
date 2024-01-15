use crate::Parser;

use wave_allocator::Box;
use wave_ast::ast::{VariableDeclaration, VariableDeclarationKind, VariableDeclarator};
use wave_diagnostics::Result;
use wave_lexer::Kind;
use wave_span::Span;

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum VariableDeclarationParent {
    Statement,
}
#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub struct VariableDeclarationContext {
    pub parent: VariableDeclarationParent,
}

impl VariableDeclarationContext {
    pub(crate) fn new(parent: VariableDeclarationParent) -> Self {
        Self { parent }
    }
}

impl<'a> Parser<'a> {
    pub(crate) fn parse_variable_declaration(
        &mut self,
        start_span: Span,
        decl_ctx: VariableDeclarationContext,
    ) -> Result<Box<'a, VariableDeclaration<'a>>> {
        let kind = match self.cur_kind() {
            Kind::Const => VariableDeclarationKind::Const,
            Kind::Let => VariableDeclarationKind::Let,
            _ => return Err(self.unexpected()),
        };
        self.bump_any();

        let mut declarations = self.ast.new_vec();

        let declaration = self.parse_variable_declarator(decl_ctx, kind)?;
        declarations.push(declaration);

        if matches!(decl_ctx.parent, VariableDeclarationParent::Statement) {
            self.asi()?;
        }

        Ok(self
            .ast
            .variable_declaration(self.end_span(start_span), kind, declarations))
    }

    fn parse_variable_declarator(
        &mut self,
        _decl_ctx: VariableDeclarationContext,
        kind: VariableDeclarationKind,
    ) -> Result<VariableDeclarator<'a>> {
        let span = self.start_span();

        let (id, definite) = self.parse_binding()?;

        let init = self
            .eat(Kind::Eq)
            .then(|| self.parse_assignment_expression_base())
            .transpose()?;

        Ok(self
            .ast
            .variable_declarator(self.end_span(span), kind, id, init, definite))
    }
}
