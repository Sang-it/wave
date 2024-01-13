#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AssignmentOperator {
    #[cfg_attr(feature = "serde", serde(rename = "="))]
    Assign,
}

impl AssignmentOperator {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Assign => "=",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum BinaryOperator {
    #[cfg_attr(feature = "serde", serde(rename = "=="))]
    Equality,
    #[cfg_attr(feature = "serde", serde(rename = "!="))]
    Inequality,
    #[cfg_attr(feature = "serde", serde(rename = "<"))]
    LessThan,
    #[cfg_attr(feature = "serde", serde(rename = "<="))]
    LessEqualThan,
    #[cfg_attr(feature = "serde", serde(rename = ">"))]
    GreaterThan,
    #[cfg_attr(feature = "serde", serde(rename = ">="))]
    GreaterEqualThan,
    #[cfg_attr(feature = "serde", serde(rename = "<<"))]
    ShiftLeft,
    #[cfg_attr(feature = "serde", serde(rename = ">>"))]
    ShiftRight,
    #[cfg_attr(feature = "serde", serde(rename = "+"))]
    Addition,
    #[cfg_attr(feature = "serde", serde(rename = "-"))]
    Subtraction,
    #[cfg_attr(feature = "serde", serde(rename = "*"))]
    Multiplication,
    #[cfg_attr(feature = "serde", serde(rename = "/"))]
    Division,
    #[cfg_attr(feature = "serde", serde(rename = "%"))]
    Remainder,
    #[cfg_attr(feature = "serde", serde(rename = "|"))]
    BitwiseOR,
    #[cfg_attr(feature = "serde", serde(rename = "^"))]
    BitwiseXOR,
    #[cfg_attr(feature = "serde", serde(rename = "&"))]
    BitwiseAnd,
    #[cfg_attr(feature = "serde", serde(rename = "**"))]
    Exponential,
}
