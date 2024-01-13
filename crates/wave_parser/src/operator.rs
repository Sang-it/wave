use wave_lexer::Kind;
use wave_syntax::{
    operator::{AssignmentOperator, BinaryOperator},
    precedence::Precedence,
};

pub fn kind_to_precedence(kind: Kind) -> Option<Precedence> {
    match kind {
        Kind::Plus | Kind::Minus => Some(Precedence::Add),
        Kind::Star | Kind::Slash | Kind::Percent => Some(Precedence::Multiply),
        Kind::Star2 => Some(Precedence::Exponential),
        _ => None,
    }
}

pub fn map_assignment_operator(kind: Kind) -> AssignmentOperator {
    match kind {
        Kind::Eq => AssignmentOperator::Assign,
        _ => unreachable!("Update Operator: {kind:?}"),
    }
}

pub fn map_binary_operator(kind: Kind) -> BinaryOperator {
    match kind {
        Kind::Plus => BinaryOperator::Addition,
        Kind::Minus => BinaryOperator::Subtraction,
        Kind::Star => BinaryOperator::Multiplication,
        Kind::Slash => BinaryOperator::Division,
        Kind::Percent => BinaryOperator::Remainder,
        Kind::Star2 => BinaryOperator::Exponential,
        _ => unreachable!("Binary Operator: {kind:?}"),
    }
}
