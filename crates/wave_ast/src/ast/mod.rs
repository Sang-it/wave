mod assignment_expression;
mod binding;
mod call_expression;
mod class;
mod expression;
mod function_declaration;
mod identifier;
mod member_expression;
mod module;
mod variable_declaration;

pub use crate::ast::function_declaration::{
    FormalParameter, FormalParameterKind, FormalParameters, Function, FunctionBody, FunctionType,
};
pub use crate::ast::variable_declaration::{
    VariableDeclaration, VariableDeclarationKind, VariableDeclarator,
};
pub use assignment_expression::{AssignmentExpression, AssignmentTarget, SimpleAssignmentTarget};
pub use binding::{BindingIdentifier, BindingPattern, BindingPatternKind};
pub use call_expression::CallExpression;
pub use class::{
    Class, ClassBody, ClassElement, ClassType, MethodDefinition, MethodDefinitionKind,
    PropertyDefinition, PropertyKey,
};
pub use expression::Expression;
pub use identifier::IdentifierReference;
pub use member_expression::{ComputedMemberExpression, MemberExpression, StaticMemberExpression};
pub use module::{
    ImportDeclaration, ImportDeclarationSpecifier, ImportSpecifier, ModuleDeclaration,
    ModuleExportName,
};

use std::hash::Hash;
use wave_allocator::{Box, Vec};
use wave_span::{Atom, Span};
use wave_syntax::operator::{BinaryOperator, LogicalOperator, UnaryOperator, UpdateOperator};

#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Debug, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize),
    serde(tag = "type", rename_all = "camelCase")
)]
pub struct Program<'a> {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    pub body: Vec<'a, Statement<'a>>,
}

impl<'a> Program<'a> {
    pub fn is_empty(&self) -> bool {
        self.body.is_empty()
    }
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
pub enum Statement<'a> {
    Declaration(Declaration<'a>),
    ExpressionStatement(Box<'a, ExpressionStatement<'a>>),
    IfStatement(Box<'a, IfStatement<'a>>),
    BlockStatement(Box<'a, BlockStatement<'a>>),
    ReturnStatement(Box<'a, ReturnStatement<'a>>),
    WhileStatement(Box<'a, WhileStatement<'a>>),
    BreakStatement(Box<'a, BreakStatement>),
    ContinueStatement(Box<'a, ContinueStatement>),
    ModuleDeclaration(Box<'a, ModuleDeclaration<'a>>),
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
pub enum Declaration<'a> {
    VariableDeclaration(Box<'a, VariableDeclaration<'a>>),
    FunctionDeclaration(Box<'a, Function<'a>>),
    ClassDeclaration(Box<'a, Class<'a>>),
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct ExpressionStatement<'a> {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    pub expression: Expression<'a>,
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct BinaryExpression<'a> {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    pub left: Expression<'a>,
    pub operator: BinaryOperator,
    pub right: Expression<'a>,
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct IfStatement<'a> {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    pub test: Expression<'a>,
    pub consequent: Statement<'a>,
    pub alternate: Option<Statement<'a>>,
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct BlockStatement<'a> {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    pub body: Vec<'a, Statement<'a>>,
}

/// Sequence Expression
#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct SequenceExpression<'a> {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    pub expressions: Vec<'a, Expression<'a>>,
}

/// Return Statement
#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct ReturnStatement<'a> {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    pub argument: Option<Expression<'a>>,
}

/// Parenthesized Expression
#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct ParenthesizedExpression<'a> {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    pub expression: Expression<'a>,
}

/// Array Expression
#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct ArrayExpression<'a> {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    pub elements: Vec<'a, ArrayExpressionElement<'a>>,
    pub trailing_comma: Option<Span>,
}

/// Array Expression Element
#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
pub enum ArrayExpressionElement<'a> {
    Expression(Expression<'a>),
}

/// Argument
#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
pub enum Argument<'a> {
    Expression(Expression<'a>),
}

/// Unary Expression
#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct UnaryExpression<'a> {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    pub operator: UnaryOperator,
    pub argument: Expression<'a>,
}

/// Update Expression
#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct UpdateExpression<'a> {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    pub operator: UpdateOperator,
    pub prefix: bool,
    pub argument: SimpleAssignmentTarget<'a>,
}

/// Binary Logical Operators
#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct LogicalExpression<'a> {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    pub left: Expression<'a>,
    pub operator: LogicalOperator,
    pub right: Expression<'a>,
}

/// While Statement
#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct WhileStatement<'a> {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    pub test: Expression<'a>,
    pub body: Statement<'a>,
}

/// Continue Statement
#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct ContinueStatement {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
}

/// Break Statement
#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct BreakStatement {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
}

/// Identifier Name
#[derive(Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct IdentifierName {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    pub name: Atom,
}

impl IdentifierName {
    pub fn new(span: Span, name: Atom) -> Self {
        Self { span, name }
    }
}

/// This Expression
#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct ThisExpression {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct Super {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct NewExpression<'a> {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    pub callee: Expression<'a>,
    pub arguments: Vec<'a, Argument<'a>>,
}
