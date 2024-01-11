use std::cell::Cell;

use wave_ast::{
    ast::{AssignmentTarget, BindingIdentifier, Expression, IdentifierReference},
    literal::{BooleanLiteral, NullLiteral, NumberLiteral, StringLiteral},
};
use wave_diagnostics::Result;
use wave_lexer::Kind;
use wave_span::{Atom, Span};
use wave_syntax::precedence::Precedence;

use crate::{diagnostics, grammar::CoverGrammar, operator::map_assignment_operator, Parser};

impl<'a> Parser<'a> {
    pub(crate) fn parse_identifier_kind(&mut self, kind: Kind) -> (Span, Atom) {
        let span = self.start_span();
        let name = self.cur_string();
        self.bump_remap(kind);
        (self.end_span(span), Atom::from(name))
    }

    pub(crate) fn parse_identifier_expression(&mut self) -> Result<Expression<'a>> {
        let ident = self.parse_identifier_reference()?;
        Ok(self.ast.identifier_reference_expression(ident))
    }

    pub(crate) fn parse_identifier_reference(&mut self) -> Result<IdentifierReference> {
        let (span, name) = self.parse_identifier_kind(Kind::Ident);
        Ok(IdentifierReference::new(span, name))
    }

    pub(crate) fn parse_expression(&mut self) -> Result<Expression<'a>> {
        let _span = self.start_span();
        let lhs = self.parse_assignment_expression_base()?;
        Ok(lhs)
    }

    pub(crate) fn parse_assignment_expression_base(&mut self) -> Result<Expression<'a>> {
        let _span = self.start_span();
        self.parse_assignment_expression()
    }

    pub(crate) fn parse_assignment_expression(&mut self) -> Result<Expression<'a>> {
        let span = self.start_span();
        let lhs = self.parse_conditional_expression()?;
        self.parse_assignment_expression_recursive(span, lhs)
    }

    fn parse_conditional_expression(&mut self) -> Result<Expression<'a>> {
        let _span = self.start_span();
        let lhs = self.parse_binary_or_logical_expression_base(Precedence::lowest())?;
        Ok(lhs)
    }

    fn parse_binary_or_logical_expression_base(
        &mut self,
        lhs_precedence: Precedence,
    ) -> Result<Expression<'a>> {
        let lhs_span = self.start_span();
        let lhs = self.parse_unary_expression_base(lhs_span)?;
        self.parse_binary_or_logical_expression_recursive(lhs_span, lhs, lhs_precedence)
    }

    fn parse_binary_or_logical_expression_recursive(
        &mut self,
        _lhs_span: Span,
        lhs: Expression<'a>,
        _min_precedence: Precedence,
    ) -> Result<Expression<'a>> {
        // Pratt Parsing Algorithm
        // <https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html>
        // TODO: implement
        Ok(lhs)
    }

    pub(crate) fn parse_unary_expression_base(&mut self, lhs_span: Span) -> Result<Expression<'a>> {
        self.parse_update_expression()
    }

    fn parse_update_expression(&mut self) -> Result<Expression<'a>> {
        let _span = self.start_span();
        let lhs = self.parse_lhs_expression()?;
        Ok(lhs)
    }

    pub(crate) fn parse_lhs_expression(&mut self) -> Result<Expression<'a>> {
        let _span = self.start_span();
        let mut _in_optional_chain = false;
        let lhs = self.parse_primary_expression()?;
        Ok(lhs)
    }

    fn parse_primary_expression(&mut self) -> Result<Expression<'a>> {
        let span = self.start_span();

        match &self.cur_kind() {
            Kind::Ident => self.parse_identifier_expression(), // fast path, keywords are checked at the end
            kind if kind.is_literal() => self.parse_literal_expression(),
            _ => self.parse_identifier_expression(),
        }
    }

    pub(crate) fn parse_literal_expression(&mut self) -> Result<Expression<'a>> {
        match self.cur_kind() {
            Kind::Str => self
                .parse_literal_string()
                .map(|literal| self.ast.literal_string_expression(literal)),
            Kind::True | Kind::False => self
                .parse_literal_boolean()
                .map(|literal| self.ast.literal_boolean_expression(literal)),
            Kind::Null => {
                let literal = self.parse_literal_null();
                Ok(self.ast.literal_null_expression(literal))
            }
            kind if kind.is_number() => self
                .parse_literal_number()
                .map(|literal| self.ast.literal_number_expression(literal)),
            _ => Err(self.unexpected()),
        }
    }

    pub(crate) fn parse_literal_boolean(&mut self) -> Result<BooleanLiteral> {
        let span = self.start_span();
        let value = match self.cur_kind() {
            Kind::True => true,
            Kind::False => false,
            _ => return Err(self.unexpected()),
        };
        self.bump_any();
        Ok(BooleanLiteral {
            span: self.end_span(span),
            value,
        })
    }

    pub(crate) fn parse_literal_number(&mut self) -> Result<NumberLiteral<'a>> {
        let span = self.start_span();
        let token = self.cur_token();
        let src = self.cur_src();
        let value = src
            .parse::<f64>()
            .map_err(|_| diagnostics::InvalidNumber("Invalid Float", token.span()))?;

        self.bump_any();
        Ok(NumberLiteral::new(self.end_span(span), value, src))
    }

    pub(crate) fn parse_literal_null(&mut self) -> NullLiteral {
        let span = self.start_span();
        self.bump_any(); // bump `null`
        NullLiteral {
            span: self.end_span(span),
        }
    }

    pub(crate) fn parse_literal_string(&mut self) -> Result<StringLiteral> {
        if !self.at(Kind::Str) {
            return Err(self.unexpected());
        }
        let value = self.cur_string();
        let span = self.start_span();
        self.bump_any();
        Ok(StringLiteral {
            span: self.end_span(span),
            value: value.into(),
        })
    }

    fn parse_assignment_expression_recursive(
        &mut self,
        span: Span,
        lhs: Expression<'a>,
    ) -> Result<Expression<'a>> {
        if !self.cur_kind().is_assignment_operator() {
            return Ok(lhs);
        }

        let operator = map_assignment_operator(self.cur_kind());

        let left = AssignmentTarget::cover(lhs, self)?;

        self.bump_any();

        let right = self.parse_assignment_expression_base()?;
        Ok(self
            .ast
            .assignment_expression(self.end_span(span), operator, left, right))
    }

    pub(crate) fn parse_binding_identifier(&mut self) -> Result<BindingIdentifier> {
        if !self.cur_kind().is_binding_identifier() {
            return Err(self.unexpected());
        }
        let (span, name) = self.parse_identifier_kind(Kind::Ident);
        Ok(BindingIdentifier {
            span,
            name,
            symbol_id: Cell::default(),
        })
    }
}
