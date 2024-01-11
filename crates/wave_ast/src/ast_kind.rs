use crate::ast::BindingIdentifier;

#[derive(Debug, Clone, Copy)]
pub enum AstKind<'a> {
    BindingIdentifier(&'a BindingIdentifier),
}
