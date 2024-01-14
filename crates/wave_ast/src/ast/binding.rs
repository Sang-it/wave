use std::cell::Cell;
use std::hash::Hash;

#[cfg(feature = "serde")]
use serde::Serialize;
use wave_allocator::Box;
use wave_span::{Atom, Span};
use wave_syntax::symbol::SymbolId;

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
