#[cfg(feature = "serde")]
use serde::Serialize;
use wave_span::Span;

use wave_allocator::Vec;

use super::{Argument, Expression};

/// Call Expression
#[derive(Debug, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize),
    serde(tag = "type", rename_all = "camelCase")
)]
pub struct CallExpression<'a> {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    pub callee: Expression<'a>,
    pub arguments: Vec<'a, Argument<'a>>,
}

impl<'a> CallExpression<'a> {}
