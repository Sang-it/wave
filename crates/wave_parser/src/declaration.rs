use crate::Parser;
use crate::{context::StatementContext, diagnostics};

use wave_allocator::Box;
use wave_ast::ast::{
    BindingPatternKind, Statement, VariableDeclaration, VariableDeclarationKind, VariableDeclarator,
};
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
    pub(crate) fn parse_let(&mut self, stmt_ctx: StatementContext) -> Result<Statement<'a>> {
        let span = self.start_span();
        let expr = self.parse_identifier_expression()?;
        self.parse_expression_statement(span, expr)
    }

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

        Ok(self
            .ast
            .variable_declaration(self.end_span(start_span), kind, declarations))
    }

    fn parse_variable_declarator(
        &mut self,
        decl_ctx: VariableDeclarationContext,
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
