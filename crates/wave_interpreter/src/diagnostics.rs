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

#[derive(Debug, Error, Diagnostic)]
#[error("Variable not found.")]
pub struct VariableNotFound(#[label] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Invalid number of arguments.")]
pub struct InvalidNumberOfArguments(#[label] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Cannot redeclare inbuilt function.")]
#[diagnostic(help("Rename the function to something else."))]
pub struct CannotRedeclareInbuiltFunction(#[label] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Cannot instantiate non-class declaration.")]
pub struct CannotInstantiateNonClass(#[label] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Cannot access property.")]
pub struct CannotAccessProperty(#[label] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Cannot call non-function expressions.")]
pub struct CannotCallNonFunction(#[label] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Cannot access non-integer index.")]
pub struct InvalidArrayAccess(#[label] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Index out of bounds.")]
pub struct IndexOutOfBounds(#[label] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("function 'push' can only be performed on arrays.")]
pub struct NotAnArray();

#[derive(Debug, Error, Diagnostic)]
#[error("Failed to import file.")]
pub struct ImportFailure();
