use std::hash::{Hash, Hasher};

use wave_span::{Atom, Span};

#[derive(Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct BooleanLiteral {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    pub value: bool,
}

impl BooleanLiteral {
    pub fn new(span: Span, value: bool) -> Self {
        Self { span, value }
    }

    pub fn as_str(&self) -> &'static str {
        if self.value {
            "true"
        } else {
            "false"
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct NullLiteral {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
}

impl Hash for NullLiteral {
    fn hash<H: Hasher>(&self, state: &mut H) {
        None::<bool>.hash(state);
    }
}

impl NullLiteral {
    pub fn new(span: Span) -> Self {
        Self { span }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct NumberLiteral<'a> {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    pub value: f64,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub raw: &'a str,
}

impl<'a> NumberLiteral<'a> {
    pub fn new(span: Span, value: f64, raw: &'a str) -> Self {
        Self { span, value, raw }
    }
}

impl<'a> Hash for NumberLiteral<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.hash(state);
    }
}

#[derive(Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct StringLiteral {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    pub value: Atom,
}

impl StringLiteral {
    pub fn new(span: Span, value: Atom) -> Self {
        Self { span, value }
    }

    /// Static Semantics: `IsStringWellFormedUnicode`
    /// test for \uD800-\uDFFF
    pub fn is_string_well_formed_unicode(&self) -> bool {
        let mut chars = self.value.chars();
        while let Some(c) = chars.next() {
            if c == '\\' && chars.next() == Some('u') {
                let hex = &chars.as_str()[..4];
                if let Ok(hex) = u32::from_str_radix(hex, 16) {
                    if (0xd800..=0xdfff).contains(&hex) {
                        return false;
                    }
                };
            }
        }
        true
    }
}
