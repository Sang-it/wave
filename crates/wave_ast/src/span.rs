use wave_span::{GetSpan, Span};

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
            Self::SequenceExpression(e) => e.span,
            Self::ParenthesizedExpression(e) => e.span,
            Self::ArrayExpression(e) => e.span,
            Self::CallExpression(e) => e.span,
            Self::UnaryExpression(e) => e.span,
            Self::UpdateExpression(e) => e.span,
            Self::LogicalExpression(e) => e.span,
        }
    }
}
