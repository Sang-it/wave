use wave_ast::ast::{ClassElement, Expression, MethodDefinition, PropertyDefinition, PropertyKey};
use wave_span::Span;

pub trait PropName {
    fn prop_name(&self) -> Option<(&str, Span)>;
}

impl<'a> PropName for ClassElement<'a> {
    fn prop_name(&self) -> Option<(&str, Span)> {
        match self {
            ClassElement::MethodDefinition(def) => def.prop_name(),
            ClassElement::PropertyDefinition(def) => def.prop_name(),
        }
    }
}

impl<'a> PropName for MethodDefinition<'a> {
    fn prop_name(&self) -> Option<(&str, Span)> {
        self.key.prop_name()
    }
}

impl<'a> PropName for PropertyDefinition<'a> {
    fn prop_name(&self) -> Option<(&str, Span)> {
        self.key.prop_name()
    }
}

impl<'a> PropName for PropertyKey<'a> {
    fn prop_name(&self) -> Option<(&str, Span)> {
        match self {
            PropertyKey::Identifier(ident) => Some((&ident.name, ident.span)),
            PropertyKey::Expression(expr) => match &expr {
                Expression::Identifier(ident) => Some((&ident.name, ident.span)),
                Expression::StringLiteral(lit) => Some((&lit.value, lit.span)),
                _ => None,
            },
        }
    }
}
