use wave_span::{GetSpan, Span};

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::ast::Expression;

impl<'a> GetSpan for Expression<'a> {
    fn span(&self) -> Span {
        match self {
            Self::BooleanLiteral(e) => e.span,
            Self::NullLiteral(e) => e.span,
            Self::NumberLiteral(e) => e.span,
            Self::StringLiteral(e) => e.span,
            Self::Identifier(e) => e.span,
            Self::AssignmentExpression(e) => e.span,
            Self::BinaryExpression(e) => e.span,
        }
    }
}
