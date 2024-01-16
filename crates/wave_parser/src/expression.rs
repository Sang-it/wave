use std::cell::Cell;

use wave_ast::{
    ast::{
        AssignmentTarget, BindingIdentifier, Expression, IdentifierName, IdentifierReference,
        SimpleAssignmentTarget,
    },
    literal::{BooleanLiteral, NullLiteral, NumberLiteral, StringLiteral},
};
use wave_diagnostics::Result;
use wave_lexer::Kind;
use wave_span::{Atom, Span};
use wave_syntax::precedence::Precedence;

use crate::{
    diagnostics,
    grammar::CoverGrammar,
    list::{ArrayExpressionList, CallArguments, SeparatedList, SequenceExpressionList},
    operator::{
        kind_to_precedence, map_assignment_operator, map_binary_operator, map_logical_operator,
        map_unary_operator, map_update_operator,
    },
    Parser,
};

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
        lhs_span: Span,
        lhs: Expression<'a>,
        min_precedence: Precedence,
    ) -> Result<Expression<'a>> {
        // Pratt Parsing Algorithm
        // <https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html>
        let mut lhs = lhs;

        loop {
            let kind = self.cur_kind();

            let Some(left_precedence) = kind_to_precedence(kind) else {
                break;
            };

            let stop = if left_precedence.is_right_associative() {
                left_precedence < min_precedence
            } else {
                left_precedence <= min_precedence
            };

            if stop {
                break;
            }

            self.bump_any(); // bump operator
            let rhs = self.parse_binary_or_logical_expression_base(left_precedence)?;

            lhs = if kind.is_logical_operator() {
                self.ast.logical_expression(
                    self.end_span(lhs_span),
                    lhs,
                    map_logical_operator(kind),
                    rhs,
                )
            } else if kind.is_binary_operator() {
                self.ast.binary_expression(
                    self.end_span(lhs_span),
                    lhs,
                    map_binary_operator(kind),
                    rhs,
                )
            } else {
                break;
            };
        }

        Ok(lhs)
    }

    pub(crate) fn parse_unary_expression_base(&mut self, lhs_span: Span) -> Result<Expression<'a>> {
        // ++ -- prefix update expressions
        if self.cur_kind().is_update_operator() {
            let operator = map_update_operator(self.cur_kind());
            self.bump_any();
            let argument = self.parse_unary_expression_base(lhs_span)?;
            let argument = SimpleAssignmentTarget::cover(argument, self)?;
            return Ok(self.ast.update_expression(
                self.end_span(lhs_span),
                operator,
                true,
                argument,
            ));
        }

        if self.cur_kind().is_unary_operator() {
            return self.parse_unary_expression();
        }

        self.parse_update_expression()
    }

    fn parse_unary_expression(&mut self) -> Result<Expression<'a>> {
        let span = self.start_span();
        let operator = map_unary_operator(self.cur_kind());
        self.bump_any();
        let argument = self.parse_unary_expression_base(span)?;
        Ok(self
            .ast
            .unary_expression(self.end_span(span), operator, argument))
    }

    fn parse_update_expression(&mut self) -> Result<Expression<'a>> {
        let span = self.start_span();
        let lhs = self.parse_lhs_expression()?;

        if self.cur_kind().is_update_operator() && !self.cur_token().is_on_new_line {
            let operator = map_update_operator(self.cur_kind());
            self.bump_any();
            let lhs = SimpleAssignmentTarget::cover(lhs, self)?;
            return Ok(self
                .ast
                .update_expression(self.end_span(span), operator, false, lhs));
        }

        Ok(lhs)
    }

    pub(crate) fn parse_lhs_expression(&mut self) -> Result<Expression<'a>> {
        let span = self.start_span();
        let lhs = self.parse_member_expression_base()?;
        let lhs = self.parse_call_expression(span, lhs)?;
        Ok(lhs)
    }

    fn parse_member_expression_base(&mut self) -> Result<Expression<'a>> {
        let span = self.start_span();
        self.parse_primary_expression()
            .and_then(|lhs| self.parse_member_expression_rhs(span, lhs))
    }

    fn parse_member_expression_rhs(
        &mut self,
        lhs_span: Span,
        lhs: Expression<'a>,
    ) -> Result<Expression<'a>> {
        let mut lhs = lhs;
        loop {
            lhs = match self.cur_kind() {
                Kind::Dot => self.parse_static_member_expression(lhs_span, lhs)?,
                Kind::LBrack => self.parse_computed_member_expression(lhs_span, lhs)?,
                _ => break,
            };
        }
        Ok(lhs)
    }

    fn parse_computed_member_expression(
        &mut self,
        lhs_span: Span,
        lhs: Expression<'a>,
    ) -> Result<Expression<'a>> {
        self.bump_any(); // advance `[`
        let property = self.parse_expression()?;
        self.expect(Kind::RBrack)?;
        Ok(self
            .ast
            .computed_member_expression(self.end_span(lhs_span), lhs, property))
    }

    fn parse_static_member_expression(
        &mut self,
        lhs_span: Span,
        lhs: Expression<'a>,
    ) -> Result<Expression<'a>> {
        self.bump_any(); // advance `.` or `?.`
        let ident = self.parse_identifier_name()?;
        Ok(self
            .ast
            .static_member_expression(self.end_span(lhs_span), lhs, ident))
    }

    pub(crate) fn parse_identifier_name(&mut self) -> Result<IdentifierName> {
        if !self.cur_kind().is_identifier_name() {
            return Err(self.unexpected());
        }
        let (span, name) = self.parse_identifier_kind(Kind::Ident);
        Ok(IdentifierName { span, name })
    }

    fn parse_call_expression(
        &mut self,
        lhs_span: Span,
        lhs: Expression<'a>,
    ) -> Result<Expression<'a>> {
        let mut lhs = lhs;
        loop {
            if self.at(Kind::LParen) {
                lhs = self.parse_call_arguments(lhs_span, lhs)?;
                continue;
            }
            break;
        }

        Ok(lhs)
    }

    fn parse_call_arguments(
        &mut self,
        lhs_span: Span,
        lhs: Expression<'a>,
    ) -> Result<Expression<'a>> {
        let call_arguments = CallArguments::parse(self)?;
        Ok(self
            .ast
            .call_expression(self.end_span(lhs_span), lhs, call_arguments.elements))
    }

    fn parse_primary_expression(&mut self) -> Result<Expression<'a>> {
        let span = self.start_span();

        match &self.cur_kind() {
            Kind::Ident => self.parse_identifier_expression(), // fast path, keywords are checked at the end
            Kind::LBrack => self.parse_array_expression(),
            Kind::LParen => self.parse_parenthesized_expression(span),
            kind if kind.is_literal() => self.parse_literal_expression(),
            _ => self.parse_identifier_expression(),
        }
    }

    pub(crate) fn parse_array_expression(&mut self) -> Result<Expression<'a>> {
        let span = self.start_span();
        let list = ArrayExpressionList::parse(self)?;
        Ok(self
            .ast
            .array_expression(self.end_span(span), list.elements, list.trailing_comma))
    }

    fn parse_parenthesized_expression(&mut self, span: Span) -> Result<Expression<'a>> {
        let list = SequenceExpressionList::parse(self)?;

        let mut expressions = list.elements;
        let paren_span = self.end_span(span);

        if expressions.is_empty() {
            return Err(diagnostics::EmptyParenthesizedExpression(paren_span).into());
        }

        let expression = if expressions.len() == 1 {
            expressions.remove(0)
        } else {
            self.ast.sequence_expression(
                Span::new(paren_span.start + 1, paren_span.end - 1),
                expressions,
            )
        };

        Ok(if self.preserve_parens {
            self.ast.parenthesized_expression(paren_span, expression)
        } else {
            expression
        })
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

    pub(crate) fn parse_paren_expression(&mut self) -> Result<Expression<'a>> {
        self.expect(Kind::LParen)?;
        let expression = self.parse_expression()?;
        self.expect(Kind::RParen)?;
        Ok(expression)
    }
}
