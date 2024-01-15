mod assignment_expression;
mod binding;
mod function_declaration;
mod identifier;
mod variable_declaration;

pub use crate::ast::function_declaration::{
    FormalParameter, FormalParameterKind, FormalParameters, Function, FunctionBody, FunctionType,
};
pub use crate::ast::variable_declaration::{
    VariableDeclaration, VariableDeclarationKind, VariableDeclarator,
};
pub use assignment_expression::{AssignmentExpression, AssignmentTarget, SimpleAssignmentTarget};
pub use binding::{BindingIdentifier, BindingPattern, BindingPatternKind};
pub use identifier::IdentifierReference;

use crate::literal::{BooleanLiteral, NullLiteral, NumberLiteral, StringLiteral};
use std::hash::Hash;
use wave_allocator::{Box, Vec};
use wave_span::Span;
use wave_syntax::operator::BinaryOperator;

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
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
pub enum Declaration<'a> {
    VariableDeclaration(Box<'a, VariableDeclaration<'a>>),
    FunctionDeclaration(Box<'a, Function<'a>>),
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
pub enum Expression<'a> {
    AssignmentExpression(Box<'a, AssignmentExpression<'a>>),
    BooleanLiteral(Box<'a, BooleanLiteral>),
    NullLiteral(Box<'a, NullLiteral>),
    NumberLiteral(Box<'a, NumberLiteral<'a>>),
    StringLiteral(Box<'a, StringLiteral>),
    Identifier(Box<'a, IdentifierReference>),
    BinaryExpression(Box<'a, BinaryExpression<'a>>),
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

/// Return Statement
#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct ReturnStatement<'a> {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    pub argument: Option<Expression<'a>>,
}
