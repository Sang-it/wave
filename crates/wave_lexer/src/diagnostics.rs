use wave_diagnostics::{
    miette::{self, Diagnostic},
    thiserror::{self, Error},
};
use wave_span::Span;

#[derive(Debug, Error, Diagnostic)]
#[error("Invalid Character `{0}`")]
pub struct InvalidCharacter(pub char, #[label] pub Span);
