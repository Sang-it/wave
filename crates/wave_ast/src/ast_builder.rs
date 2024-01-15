use crate::ast::{
    Argument, ArrayExpression, ArrayExpressionElement, AssignmentExpression, AssignmentTarget,
    BinaryExpression, BindingIdentifier, BindingPattern, BindingPatternKind, BlockStatement,
    CallExpression, Declaration, Expression, ExpressionStatement, FormalParameter,
    FormalParameterKind, FormalParameters, Function, FunctionBody, FunctionType,
    IdentifierReference, IfStatement, LogicalExpression, ParenthesizedExpression, Program,
    ReturnStatement, SequenceExpression, SimpleAssignmentTarget, Statement, UnaryExpression,
    UpdateExpression, VariableDeclaration, VariableDeclarationKind, VariableDeclarator,
    WhileStatement,
};
use crate::literal::{BooleanLiteral, NullLiteral, NumberLiteral, StringLiteral};
use wave_allocator::{Allocator, Box, Vec};
use wave_span::Span;
use wave_syntax::operator::{
    AssignmentOperator, BinaryOperator, LogicalOperator, UnaryOperator, UpdateOperator,
};

pub struct AstBuilder<'a> {
    pub allocator: &'a Allocator,
}

impl<'a> AstBuilder<'a> {
    pub fn new(allocator: &'a Allocator) -> Self {
        Self { allocator }
    }

    pub fn program(&self, span: Span, body: Vec<'a, Statement<'a>>) -> Program<'a> {
        Program { span, body }
    }

    #[inline]
    pub fn new_vec<T>(&self) -> Vec<'a, T> {
        Vec::new_in(self.allocator)
    }

    #[inline]
    pub fn alloc<T>(&self, value: T) -> Box<'a, T> {
        Box(self.allocator.alloc(value))
    }

    pub fn number_literal(&self, span: Span, value: f64, raw: &'a str) -> NumberLiteral<'a> {
        NumberLiteral { span, value, raw }
    }

    pub fn boolean_literal(&self, span: Span, value: bool) -> BooleanLiteral {
        BooleanLiteral { span, value }
    }

    pub fn literal_string_expression(&self, literal: StringLiteral) -> Expression<'a> {
        Expression::StringLiteral(self.alloc(literal))
    }

    pub fn literal_boolean_expression(&self, literal: BooleanLiteral) -> Expression<'a> {
        Expression::BooleanLiteral(self.alloc(literal))
    }

    pub fn literal_null_expression(&self, literal: NullLiteral) -> Expression<'a> {
        Expression::NullLiteral(self.alloc(literal))
    }

    pub fn literal_number_expression(&self, literal: NumberLiteral<'a>) -> Expression<'a> {
        Expression::NumberLiteral(self.alloc(literal))
    }

    pub fn variable_declaration(
        &self,
        span: Span,
        kind: VariableDeclarationKind,
        declarations: Vec<'a, VariableDeclarator<'a>>,
    ) -> Box<'a, VariableDeclaration<'a>> {
        self.alloc(VariableDeclaration {
            span,
            kind,
            declarations,
        })
    }

    pub fn variable_declarator(
        &self,
        span: Span,
        kind: VariableDeclarationKind,
        id: BindingPattern<'a>,
        init: Option<Expression<'a>>,
        definite: bool,
    ) -> VariableDeclarator<'a> {
        VariableDeclarator {
            span,
            kind,
            id,
            init,
            definite,
        }
    }

    pub fn binding_pattern(&self, kind: BindingPatternKind<'a>) -> BindingPattern<'a> {
        BindingPattern { kind }
    }

    pub fn binding_pattern_identifier(
        &self,
        identifier: BindingIdentifier,
    ) -> BindingPatternKind<'a> {
        BindingPatternKind::BindingIdentifier(self.alloc(identifier))
    }

    pub fn identifier_reference_expression(&self, ident: IdentifierReference) -> Expression<'a> {
        Expression::Identifier(self.alloc(ident))
    }

    pub fn assignment_expression(
        &self,
        span: Span,
        operator: AssignmentOperator,
        left: AssignmentTarget<'a>,
        right: Expression<'a>,
    ) -> Expression<'a> {
        Expression::AssignmentExpression(self.alloc(AssignmentExpression {
            span,
            operator,
            left,
            right,
        }))
    }

    pub fn expression_statement(&self, span: Span, expression: Expression<'a>) -> Statement<'a> {
        Statement::ExpressionStatement(self.alloc(ExpressionStatement { span, expression }))
    }

    pub fn binary_expression(
        &self,
        span: Span,
        left: Expression<'a>,
        operator: BinaryOperator,
        right: Expression<'a>,
    ) -> Expression<'a> {
        Expression::BinaryExpression(self.alloc(BinaryExpression {
            span,
            left,
            operator,
            right,
        }))
    }

    pub fn if_statement(
        &self,
        span: Span,
        test: Expression<'a>,
        consequent: Statement<'a>,
        alternate: Option<Statement<'a>>,
    ) -> Statement<'a> {
        Statement::IfStatement(self.alloc(IfStatement {
            span,
            test,
            consequent,
            alternate,
        }))
    }

    pub fn block(&self, span: Span, body: Vec<'a, Statement<'a>>) -> Box<'a, BlockStatement<'a>> {
        self.alloc(BlockStatement { span, body })
    }

    pub fn block_statement(&self, block: Box<'a, BlockStatement<'a>>) -> Statement<'a> {
        Statement::BlockStatement(self.alloc(BlockStatement {
            span: block.span,
            body: block.unbox().body,
        }))
    }

    pub fn function_declaration(&self, func: Box<'a, Function<'a>>) -> Statement<'a> {
        Statement::Declaration(Declaration::FunctionDeclaration(func))
    }

    pub fn formal_parameter(&self, span: Span, pattern: BindingPattern<'a>) -> FormalParameter<'a> {
        FormalParameter { span, pattern }
    }

    pub fn formal_parameters(
        &self,
        span: Span,
        kind: FormalParameterKind,
        items: Vec<'a, FormalParameter<'a>>,
    ) -> Box<'a, FormalParameters<'a>> {
        self.alloc(FormalParameters { span, kind, items })
    }

    pub fn function(
        &self,
        r#type: FunctionType,
        span: Span,
        id: Option<BindingIdentifier>,
        params: Box<'a, FormalParameters<'a>>,
        body: Option<Box<'a, FunctionBody<'a>>>,
    ) -> Box<'a, Function<'a>> {
        self.alloc(Function {
            r#type,
            span,
            id,
            params,
            body,
        })
    }

    pub fn function_body(
        &self,
        span: Span,
        statements: Vec<'a, Statement<'a>>,
    ) -> Box<'a, FunctionBody<'a>> {
        self.alloc(FunctionBody { span, statements })
    }

    pub fn return_statement(&self, span: Span, argument: Option<Expression<'a>>) -> Statement<'a> {
        Statement::ReturnStatement(self.alloc(ReturnStatement { span, argument }))
    }

    pub fn sequence_expression(
        &self,
        span: Span,
        expressions: Vec<'a, Expression<'a>>,
    ) -> Expression<'a> {
        Expression::SequenceExpression(self.alloc(SequenceExpression { span, expressions }))
    }

    pub fn parenthesized_expression(
        &self,
        span: Span,
        expression: Expression<'a>,
    ) -> Expression<'a> {
        Expression::ParenthesizedExpression(
            self.alloc(ParenthesizedExpression { span, expression }),
        )
    }

    pub fn array_expression(
        &self,
        span: Span,
        elements: Vec<'a, ArrayExpressionElement<'a>>,
        trailing_comma: Option<Span>,
    ) -> Expression<'a> {
        Expression::ArrayExpression(self.alloc(ArrayExpression {
            span,
            elements,
            trailing_comma,
        }))
    }

    pub fn call_expression(
        &self,
        span: Span,
        callee: Expression<'a>,
        arguments: Vec<'a, Argument<'a>>,
    ) -> Expression<'a> {
        Expression::CallExpression(self.alloc(CallExpression {
            span,
            callee,
            arguments,
        }))
    }

    pub fn unary_expression(
        &self,
        span: Span,
        operator: UnaryOperator,
        argument: Expression<'a>,
    ) -> Expression<'a> {
        Expression::UnaryExpression(self.alloc(UnaryExpression {
            span,
            operator,
            argument,
        }))
    }

    pub fn update_expression(
        &self,
        span: Span,
        operator: UpdateOperator,
        prefix: bool,
        argument: SimpleAssignmentTarget<'a>,
    ) -> Expression<'a> {
        Expression::UpdateExpression(self.alloc(UpdateExpression {
            span,
            operator,
            prefix,
            argument,
        }))
    }

    pub fn logical_expression(
        &self,
        span: Span,
        left: Expression<'a>,
        operator: LogicalOperator,
        right: Expression<'a>,
    ) -> Expression<'a> {
        Expression::LogicalExpression(self.alloc(LogicalExpression {
            span,
            left,
            operator,
            right,
        }))
    }

    pub fn while_statement(
        &self,
        span: Span,
        test: Expression<'a>,
        body: Statement<'a>,
    ) -> Statement<'a> {
        Statement::WhileStatement(self.alloc(WhileStatement { span, test, body }))
    }
}
