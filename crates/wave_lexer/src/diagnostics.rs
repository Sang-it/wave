use wave_diagnostics::{
    miette::{self, Diagnostic},
    thiserror::{self, Error},
};
use wave_span::Span;

#[derive(Debug, Error, Diagnostic)]
#[error("Unterminated string")]
pub struct UnterminatedString(#[label] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Invalid Character `{0}`")]
pub struct InvalidCharacter(pub char, #[label] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Invalid characters after number")]
pub struct InvalidNumberEnd(#[label] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unterminated multiline comment")]
pub struct UnterminatedMultiLineComment(#[label] pub Span);
