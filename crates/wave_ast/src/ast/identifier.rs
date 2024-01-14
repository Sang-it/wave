#[cfg(feature = "serde")]
use serde::Serialize;

use wave_span::{Atom, Span};
use wave_syntax::reference::{ReferenceFlag, ReferenceId};

use std::{cell::Cell, hash::Hash};
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
