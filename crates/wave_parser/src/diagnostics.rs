use wave_diagnostics::{
    miette::{self, Diagnostic},
    thiserror::{self, Error},
};

use wave_span::Span;

#[derive(Debug, Error, Diagnostic)]
#[error("Lexical declaration cannot appear in a single-statement context")]
#[diagnostic(help("Wrap this declaration in a block statement"))]
pub struct LexicalDeclarationSingleStatement(#[label] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Invalid Number {0}")]
pub struct InvalidNumber(pub &'static str, #[label] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected token")]
#[diagnostic()]
pub struct UnexpectedToken(#[label] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Cannot assign to this expression")]
#[diagnostic()]
pub struct InvalidAssignment(#[label] pub Span);
