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
#[error("TS2681: A constructor cannot have a `this` parameter.")]
#[diagnostic()]
pub struct TSConstructorThisParameter(#[label] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Invalid class declaration")]
#[diagnostic(help("Classes can only be declared at top level or inside a block"))]
pub struct ClassDeclaration(#[label] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Invalid Number {0}")]
pub struct InvalidNumber(pub &'static str, #[label] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected token")]
#[diagnostic()]
pub struct UnexpectedToken(#[label] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("'super' can only be used with function calls or in property accesses")]
#[diagnostic(help("replace with `super()` or `super.prop` or `super[prop]`"))]
pub struct UnexpectedSuper(#[label] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Cannot assign to this expression")]
#[diagnostic()]
pub struct InvalidAssignment(#[label] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected a semicolon or an implicit semicolon after a statement, but found none")]
#[diagnostic(help("Try insert a semicolon here"))]
pub struct AutoSemicolonInsertion(#[label] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected `{0}` but found `{1}`")]
#[diagnostic()]
pub struct ExpectToken(
    pub &'static str,
    pub &'static str,
    #[label("`{0}` expected")] pub Span,
);

#[derive(Debug, Error, Diagnostic)]
#[error("Expected function name")]
#[diagnostic(help("Function name is required in function declaration or named export"))]
pub struct ExpectFunctionName(#[label] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("TS1108: A 'return' statement can only be used within a function body")]
#[diagnostic()]
pub struct ReturnStatementOnlyInFunctionBody(#[label] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Empty parenthesized expression")]
#[diagnostic()]
pub struct EmptyParenthesizedExpression(#[label] pub Span);

#[derive(Debug, Error, Diagnostic)]
#[error("Classes can't have a field named 'constructor'")]
#[diagnostic()]
pub struct FieldConstructor(#[label] pub Span);
