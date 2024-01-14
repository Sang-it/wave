use std::cell::Cell;

use wave_allocator::Box;
use wave_ast::ast::{
    BindingIdentifier, FormalParameterKind, FormalParameters, Function, FunctionBody, FunctionType,
    Statement,
};
use wave_diagnostics::Result;
use wave_lexer::Kind;
use wave_span::Span;

use crate::{
    context::StatementContext,
    diagnostics,
    list::{FormalParameterList, SeparatedList},
    Parser,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FunctionKind {
    Declaration { single_statement: bool },
}

impl FunctionKind {
    pub(crate) fn is_id_required(self) -> bool {
        matches!(
            self,
            Self::Declaration {
                single_statement: true
            }
        )
    }
}

impl<'a> Parser<'a> {
    pub(crate) fn at_function(&mut self) -> bool {
        self.at(Kind::Function)
    }

    pub(crate) fn parse_function(
        &mut self,
        span: Span,
        id: Option<BindingIdentifier>,
        func_kind: FunctionKind,
    ) -> Result<Box<'a, Function<'a>>> {
        let params = self.parse_formal_parameters(FormalParameterKind::FormalParameter)?;

        let body = if self.at(Kind::LCurly) {
            Some(self.parse_function_body()?)
        } else {
            None
        };

        let function_type = match func_kind {
            FunctionKind::Declaration { .. } => FunctionType::FunctionDeclaration,
        };

        Ok(self
            .ast
            .function(function_type, self.end_span(span), id, params, body))
    }

    pub(crate) fn parse_function_body(&mut self) -> Result<Box<'a, FunctionBody<'a>>> {
        let span = self.start_span();
        self.expect(Kind::LCurly)?;
        let statements = self.parse_statements()?;
        self.expect(Kind::RCurly)?;
        Ok(self.ast.function_body(self.end_span(span), statements))
    }

    pub(crate) fn parse_formal_parameters(
        &mut self,
        params_kind: FormalParameterKind,
    ) -> Result<Box<'a, FormalParameters<'a>>> {
        let span = self.start_span();
        let list: FormalParameterList<'_> = FormalParameterList::parse(self)?;
        let formal_parameters =
            self.ast
                .formal_parameters(self.end_span(span), params_kind, list.elements);
        Ok(formal_parameters)
    }

    pub(crate) fn parse_function_declaration(
        &mut self,
        stmt_ctx: StatementContext,
    ) -> Result<Statement<'a>> {
        let func_kind = FunctionKind::Declaration {
            single_statement: stmt_ctx.is_single_statement(),
        };

        let decl = self.parse_function_impl(func_kind)?;

        Ok(self.ast.function_declaration(decl))
    }

    pub(crate) fn parse_function_impl(
        &mut self,
        func_kind: FunctionKind,
    ) -> Result<Box<'a, Function<'a>>> {
        let span = self.start_span();
        self.expect(Kind::Function)?;
        let id = self.parse_function_id(func_kind);
        self.parse_function(span, id, func_kind)
    }

    pub(crate) fn parse_function_id(&mut self, kind: FunctionKind) -> Option<BindingIdentifier> {
        let id = self.cur_kind().is_binding_identifier().then(|| {
            let (span, name) = self.parse_identifier_kind(Kind::Ident);
            BindingIdentifier {
                span,
                name,
                symbol_id: Cell::default(),
            }
        });

        if kind.is_id_required() && id.is_none() {
            self.error(diagnostics::ExpectFunctionName(self.cur_token().span()));
        }

        id
    }
}

