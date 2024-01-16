use crate::ast::{BindingIdentifier, Expression, Function, IdentifierName};
use wave_allocator::{Box, Vec};

#[cfg(feature = "serde")]
use serde::Serialize;
use wave_span::Span;

/// Class Definitions
#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "camelCase"))]
pub struct Class<'a> {
    pub r#type: ClassType,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    pub id: Option<BindingIdentifier>,
    pub super_class: Option<Expression<'a>>,
    pub body: Box<'a, ClassBody<'a>>,
}

impl<'a> Class<'a> {
    pub fn is_expression(&self) -> bool {
        self.r#type == ClassType::ClassExpression
    }

    pub fn is_declaration(&self) -> bool {
        self.r#type == ClassType::ClassDeclaration
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum ClassType {
    ClassDeclaration,
    ClassExpression,
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct ClassBody<'a> {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    pub body: Vec<'a, ClassElement<'a>>,
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
pub enum ClassElement<'a> {
    MethodDefinition(Box<'a, MethodDefinition<'a>>),
    PropertyDefinition(Box<'a, PropertyDefinition<'a>>),
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct MethodDefinition<'a> {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    pub key: PropertyKey<'a>,
    pub value: Box<'a, Function<'a>>, // FunctionExpression
    pub kind: MethodDefinitionKind,
}

#[derive(Debug, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize),
    serde(tag = "type", rename_all = "camelCase")
)]
pub struct PropertyDefinition<'a> {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    pub key: PropertyKey<'a>,
    pub value: Option<Expression<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "lowercase"))]
pub enum MethodDefinitionKind {
    Constructor,
    Method,
}

impl MethodDefinitionKind {
    pub fn is_constructor(&self) -> bool {
        matches!(self, Self::Constructor)
    }
    pub fn is_method(&self) -> bool {
        matches!(self, Self::Method)
    }
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
pub enum PropertyKey<'a> {
    Identifier(Box<'a, IdentifierName>),
    Expression(Expression<'a>),
}
