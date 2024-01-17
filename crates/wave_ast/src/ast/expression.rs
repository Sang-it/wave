#[cfg(feature = "serde")]
use serde::Serialize;

use crate::ast::{
    AssignmentExpression, BinaryExpression, IdentifierReference, ParenthesizedExpression,
    SequenceExpression,
};
use crate::{BooleanLiteral, NullLiteral, NumberLiteral, StringLiteral};
use wave_allocator::Box;

use super::{
    ArrayExpression, CallExpression, LogicalExpression, MemberExpression, NewExpression, Super,
    ThisExpression, UnaryExpression, UpdateExpression,
};

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
pub enum Expression<'a> {
    AssignmentExpression(Box<'a, AssignmentExpression<'a>>),
    BooleanLiteral(Box<'a, BooleanLiteral>),
    NullLiteral(Box<'a, NullLiteral>),
    NumberLiteral(Box<'a, NumberLiteral<'a>>),
    StringLiteral(Box<'a, StringLiteral>),
    Identifier(Box<'a, IdentifierReference>),
    BinaryExpression(Box<'a, BinaryExpression<'a>>),
    SequenceExpression(Box<'a, SequenceExpression<'a>>),
    ParenthesizedExpression(Box<'a, ParenthesizedExpression<'a>>),
    ArrayExpression(Box<'a, ArrayExpression<'a>>),
    CallExpression(Box<'a, CallExpression<'a>>),
    UnaryExpression(Box<'a, UnaryExpression<'a>>),
    UpdateExpression(Box<'a, UpdateExpression<'a>>),
    LogicalExpression(Box<'a, LogicalExpression<'a>>),
    MemberExpression(Box<'a, MemberExpression<'a>>),
    ThisExpression(Box<'a, ThisExpression>),
    Super(Box<'a, Super>),
    NewExpression(Box<'a, NewExpression<'a>>),
}

impl<'a> Expression<'a> {
    pub fn is_primary_expression(&self) -> bool {
        self.is_literal()
            || matches!(
                self,
                Self::Identifier(_)
                    | Self::ThisExpression(_)
                    | Self::ParenthesizedExpression(_)
                    | Self::ArrayExpression(_)
            )
    }

    pub fn get_literal_value(&self) -> &str {
        match self {
            Self::BooleanLiteral(lit) => {
                if lit.value {
                    "true"
                } else {
                    "false"
                }
            }
            Self::NullLiteral(_) => "null",
            Self::NumberLiteral(lit) => lit.raw,
            Self::StringLiteral(lit) => &lit.value,
            Self::Identifier(ident) => &ident.name,
            _ => "",
        }
    }

    pub fn is_literal(&self) -> bool {
        matches!(
            self,
            Self::BooleanLiteral(_)
                | Self::NullLiteral(_)
                | Self::NumberLiteral(_)
                | Self::StringLiteral(_)
        )
    }

    pub fn is_number_literal(&self) -> bool {
        matches!(self, Self::NumberLiteral(_))
    }

    pub fn is_string_literal(&self) -> bool {
        matches!(self, Self::StringLiteral(_))
    }

    pub fn is_specific_string_literal(&self, string: &str) -> bool {
        match self {
            Self::StringLiteral(s) => s.value == string,
            _ => false,
        }
    }

    /// Determines whether the given expr is a `null` literal
    pub fn is_null(&self) -> bool {
        matches!(self, Expression::NullLiteral(_))
    }

    /// Determines whether the given expr is a `undefined` literal
    pub fn is_undefined(&self) -> bool {
        matches!(self, Self::Identifier(ident) if ident.name == "undefined")
    }

    /// Determines whether the given expr is a certain number
    pub fn is_number(&self, val: f64) -> bool {
        matches!(self, Self::NumberLiteral(lit) if (lit.value - val).abs() < f64::EPSILON)
    }

    /// Determines whether the given numeral literal's raw value is exactly val
    pub fn is_specific_raw_number_literal(&self, val: &str) -> bool {
        matches!(self, Self::NumberLiteral(lit) if lit.raw == val)
    }

    /// Determines whether the given expr is a `null` or `undefined` or `void 0`
    pub fn is_null_or_undefined(&self) -> bool {
        self.is_null()
    }

    /// Determines whether the given expr is a `NaN` literal
    pub fn is_nan(&self) -> bool {
        matches!(self, Self::Identifier(ident) if ident.name == "NaN")
    }

    pub fn is_specific_id(&self, name: &str) -> bool {
        match self.get_inner_expression() {
            Expression::Identifier(ident) => ident.name == name,
            _ => false,
        }
    }

    pub fn get_inner_expression(&self) -> &Expression<'a> {
        match self {
            Expression::ParenthesizedExpression(expr) => expr.expression.get_inner_expression(),
            _ => self,
        }
    }

    pub fn is_identifier_reference(&self) -> bool {
        matches!(self, Expression::Identifier(_))
    }

    pub fn get_identifier_reference(&self) -> Option<&IdentifierReference> {
        match self.get_inner_expression() {
            Expression::Identifier(ident) => Some(ident),
            _ => None,
        }
    }

    pub fn is_call_expression(&self) -> bool {
        matches!(self, Expression::CallExpression(_))
    }

    pub fn is_call_like_expression(&self) -> bool {
        self.is_call_expression() && matches!(self, Expression::NewExpression(_))
    }

    pub fn is_binaryish(&self) -> bool {
        matches!(
            self,
            Expression::BinaryExpression(_) | Expression::LogicalExpression(_)
        )
    }

    /// Returns literal's value converted to the Boolean type
    /// returns `true` when node is truthy, `false` when node is falsy, `None` when it cannot be determined.
    pub fn get_boolean_value(&self) -> Option<bool> {
        match self {
            Self::BooleanLiteral(lit) => Some(lit.value),
            Self::NullLiteral(_) => Some(false),
            Self::NumberLiteral(lit) => Some(lit.value != 0.0),
            Self::StringLiteral(lit) => Some(!lit.value.is_empty()),
            _ => None,
        }
    }

    pub fn get_member_expr(&self) -> Option<&MemberExpression<'a>> {
        match self.get_inner_expression() {
            Expression::MemberExpression(member_expr) => Some(member_expr),
            _ => None,
        }
    }

    pub fn is_immutable_value(&self) -> bool {
        match self {
            Self::BooleanLiteral(_)
            | Self::NullLiteral(_)
            | Self::NumberLiteral(_)
            | Self::StringLiteral(_) => true,
            Self::UnaryExpression(unary_expr) => unary_expr.argument.is_immutable_value(),
            Self::Identifier(ident) => {
                matches!(ident.name.as_str(), "undefined" | "Infinity" | "NaN")
            }
            _ => false,
        }
    }
}
