use wave_allocator::{Box, Vec};
use wave_ast::ast::{
    Class, ClassBody, ClassElement, ClassType, Expression, MethodDefinition, MethodDefinitionKind,
    PropertyDefinition, PropertyKey, Statement,
};
use wave_diagnostics::Result;
use wave_lexer::Kind;
use wave_span::Span;

use crate::{
    context::StatementContext,
    diagnostics,
    list::{ClassElements, NormalList},
    syntax_directed_operations::prop_name::PropName,
    Parser,
};

type Extends<'a> = Vec<'a, (Expression<'a>, Span)>;

impl<'a> Parser<'a> {
    // `start_span` points at the start of all decoractors and `class` keyword.
    pub(crate) fn parse_class_statement(
        &mut self,
        stmt_ctx: StatementContext,
        start_span: Span,
    ) -> Result<Statement<'a>> {
        let decl = self.parse_class_declaration(start_span)?;

        if stmt_ctx.is_single_statement() {
            self.error(diagnostics::ClassDeclaration(Span::new(
                decl.span.start,
                decl.body.span.start,
            )));
        }

        Ok(self.ast.class_declaration(decl))
    }

    pub(crate) fn parse_class_declaration(
        &mut self,
        start_span: Span,
    ) -> Result<Box<'a, Class<'a>>> {
        self.parse_class(start_span, ClassType::ClassDeclaration)
    }

    fn parse_class(&mut self, start_span: Span, r#type: ClassType) -> Result<Box<'a, Class<'a>>> {
        self.bump_any();

        let id = if self.cur_kind().is_binding_identifier() {
            Some(self.parse_binding_identifier()?)
        } else {
            None
        };

        let extends = self.parse_heritage_clause()?;

        let mut super_class = None;

        if let Some(mut extends) = extends {
            if !extends.is_empty() {
                let first_extends = extends.remove(0);
                super_class = Some(first_extends.0);
            }
        }

        let body = self.parse_class_body()?;

        Ok(self
            .ast
            .class(r#type, self.end_span(start_span), id, super_class, body))
    }

    fn parse_class_body(&mut self) -> Result<Box<'a, ClassBody<'a>>> {
        let span = self.start_span();
        let mut class_elements = ClassElements::new(self);
        class_elements.parse(self)?;
        let body = class_elements.elements;
        Ok(self.ast.class_body(self.end_span(span), body))
    }

    pub(crate) fn parse_heritage_clause(&mut self) -> Result<Option<Extends<'a>>> {
        let mut extends = None;

        // while let Kind::Extends = self.cur_kind() {
        //     extends = Some(self.parse_extends_clause()?);
        // }

        loop {
            match self.cur_kind() {
                Kind::Extends => {
                    extends = Some(self.parse_extends_clause()?);
                }
                _ => break,
            }
        }

        Ok(extends)
    }

    fn parse_extends_clause(&mut self) -> Result<Extends<'a>> {
        self.bump_any(); // bump `extends`
        let mut extends = self.ast.new_vec();

        let span = self.start_span();
        let first_extends = self.parse_lhs_expression()?;

        extends.push((first_extends, self.end_span(span)));

        while self.eat(Kind::Comma) {
            let span = self.start_span();
            let extend = self.parse_lhs_expression()?;
            extends.push((extend, self.end_span(span)));
        }

        Ok(extends)
    }

    pub(crate) fn parse_class_element(&mut self) -> Result<ClassElement<'a>> {
        let span = self.start_span();

        let kind = MethodDefinitionKind::Method;

        let _ = self.peek_kind().is_class_element_name_start();

        let key = match self.cur_kind() {
            kind if kind.is_class_element_name_start() => self.parse_class_element_name()?,
            _ => return Err(self.unexpected()),
        };

        if self.at(Kind::LParen) {
            let definition = self.parse_class_method_definition(span, kind, key)?;
            Ok(definition)
        } else {
            if !kind.is_method() {
                return Err(self.unexpected());
            }
            let definition = self.parse_class_property_definition(span, key)?;
            if let Some((name, span)) = definition.prop_name() {
                if name == "constructor" {
                    self.error(diagnostics::FieldConstructor(span));
                }
            }
            Ok(definition)
        }
    }

    fn parse_class_property_definition(
        &mut self,
        span: Span,
        key: PropertyKey<'a>,
    ) -> Result<ClassElement<'a>> {
        let value = if self.eat(Kind::Eq) {
            // let current_flags = self.scope.current_flags();
            // self.scope.set_current_flags(self.scope.current_flags());
            let expr = self.parse_expression()?;
            // self.scope.set_current_flags(current_flags);
            Some(expr)
        } else {
            None
        };
        self.asi()?;

        let property_definition = PropertyDefinition {
            span: self.end_span(span),
            key,
            value,
        };

        Ok(ClassElement::PropertyDefinition(
            self.ast.alloc(property_definition),
        ))
    }

    fn parse_class_method_definition(
        &mut self,
        span: Span,
        kind: MethodDefinitionKind,
        key: PropertyKey<'a>,
    ) -> Result<ClassElement<'a>> {
        let kind = if key
            .prop_name()
            .map_or(false, |(name, _)| name == "constructor")
        {
            MethodDefinitionKind::Constructor
        } else {
            kind
        };

        let value = self.parse_method()?;

        let method_definition = MethodDefinition {
            span: self.end_span(span),
            key,
            value,
            kind,
        };

        Ok(ClassElement::MethodDefinition(
            self.ast.alloc(method_definition),
        ))
    }

    fn parse_class_element_name(&mut self) -> Result<PropertyKey<'a>> {
        self.parse_property_name()
    }
}
