use crate::literal::{BooleanLiteral, NullLiteral, NumberLiteral, StringLiteral};
use std::{cell::Cell, fmt, hash::Hash};
use wave_allocator::{Box, Vec};
use wave_span::{Atom, Span};
use wave_syntax::{
    operator::AssignmentOperator,
    reference::{ReferenceFlag, ReferenceId},
    symbol::SymbolId,
};

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
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
pub enum Declaration<'a> {
    VariableDeclaration(Box<'a, VariableDeclaration<'a>>),
}

/// Variable Declaration
#[derive(Debug, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize),
    serde(tag = "type", rename_all = "camelCase")
)]
pub struct VariableDeclaration<'a> {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    pub kind: VariableDeclarationKind,
    pub declarations: Vec<'a, VariableDeclarator<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "lowercase"))]
pub enum VariableDeclarationKind {
    Const,
    Let,
}

impl VariableDeclarationKind {
    pub fn is_const(&self) -> bool {
        matches!(self, Self::Const)
    }

    pub fn is_lexical(&self) -> bool {
        matches!(self, Self::Const | Self::Let)
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Const => "const",
            Self::Let => "let",
        }
    }
}

impl fmt::Display for VariableDeclarationKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.as_str();
        write!(f, "{s}")
    }
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct VariableDeclarator<'a> {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub kind: VariableDeclarationKind,
    pub id: BindingPattern<'a>,
    pub init: Option<Expression<'a>>,
    pub definite: bool,
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
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct AssignmentExpression<'a> {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    pub operator: AssignmentOperator,
    pub left: AssignmentTarget<'a>,
    pub right: Expression<'a>,
}

/// Expression Statement
#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct ExpressionStatement<'a> {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    pub expression: Expression<'a>,
}

#[derive(Debug, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize),
    serde(tag = "type", rename_all = "camelCase")
)]
pub struct BindingPattern<'a> {
    pub kind: BindingPatternKind<'a>,
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
pub enum BindingPatternKind<'a> {
    /// `const a = 1`
    BindingIdentifier(Box<'a, BindingIdentifier>),
}

/// Binding Identifier
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct BindingIdentifier {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    pub name: Atom,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub symbol_id: Cell<Option<SymbolId>>,
}

impl Hash for BindingIdentifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.span.hash(state);
        self.name.hash(state);
    }
}

impl BindingIdentifier {
    pub fn new(span: Span, name: Atom) -> Self {
        Self {
            span,
            name,
            symbol_id: Cell::default(),
        }
    }
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
pub enum AssignmentTarget<'a> {
    SimpleAssignmentTarget(SimpleAssignmentTarget<'a>),
}

impl<'a> AssignmentTarget<'a> {
    pub fn is_simple(&self) -> bool {
        matches!(self, Self::SimpleAssignmentTarget(_))
    }

    pub fn is_identifier(&self) -> bool {
        matches!(
            self,
            Self::SimpleAssignmentTarget(SimpleAssignmentTarget::AssignmentTargetIdentifier(_))
        )
    }
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
pub enum SimpleAssignmentTarget<'a> {
    AssignmentTargetIdentifier(Box<'a, IdentifierReference>),
}

/// Identifier Reference
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct IdentifierReference {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    pub name: Atom,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub reference_id: Cell<Option<ReferenceId>>,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub reference_flag: ReferenceFlag,
}

impl Hash for IdentifierReference {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.span.hash(state);
        self.name.hash(state);
    }
}

impl IdentifierReference {
    pub fn new(span: Span, name: Atom) -> Self {
        Self {
            span,
            name,
            reference_id: Cell::default(),
            reference_flag: ReferenceFlag::default(),
        }
    }
}
