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
#[error("Types are mismatched.")]
#[diagnostic(help("This operation can only be performed on same types."))]
pub struct MismatchedTypes();
