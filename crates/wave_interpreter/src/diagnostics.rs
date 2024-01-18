use wave_diagnostics::{
    miette::{self, Diagnostic},
    thiserror::{self, Error},
};

use wave_span::Span;

#[derive(Debug, Error, Diagnostic)]
#[error("Not a number.")]
#[diagnostic(help("This operation can only be performed on numbers."))]
pub struct InvalidNumber(#[label] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Operator not implemented {0}")]
#[diagnostic(help("Will be implemented in the future."))]
pub struct OperatorNotImplemented(pub &'static str, #[label] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Not a boolean.")]
#[diagnostic(help("This operation can only be performed on booleans."))]
pub struct InvalidBoolean(#[label] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Type mismatch.")]
#[diagnostic(help("This operation can only be performed on expressions with same type."))]
pub struct TypeMismatch(#[label] pub Span);
