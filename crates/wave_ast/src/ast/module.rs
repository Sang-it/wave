use crate::StringLiteral;
use wave_allocator::{Box, Vec};
use wave_span::{Atom, Span};

#[cfg(feature = "serde")]
use serde::Serialize;

use super::IdentifierName;

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
pub enum ModuleDeclaration<'a> {
    ImportDeclaration(Box<'a, ImportDeclaration<'a>>),
}

#[derive(Debug, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize),
    serde(tag = "type", rename_all = "camelCase")
)]
pub struct ImportDeclaration<'a> {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    pub specifiers: Option<Vec<'a, ImportDeclarationSpecifier>>,
    pub source: StringLiteral,
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
pub enum ImportDeclarationSpecifier {
    ImportSpecifier(ImportSpecifier),
}

#[derive(Debug, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize),
    serde(tag = "type", rename_all = "camelCase")
)]
pub struct ImportSpecifier {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    pub imported: ModuleExportName,
}

#[derive(Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
pub enum ModuleExportName {
    Identifier(IdentifierName),
}

impl ModuleExportName {
    pub fn name(&self) -> &Atom {
        match self {
            Self::Identifier(identifier) => &identifier.name,
        }
    }
}
