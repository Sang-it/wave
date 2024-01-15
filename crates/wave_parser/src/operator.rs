use wave_lexer::Kind;
use wave_syntax::{
    operator::{
        AssignmentOperator, BinaryOperator, LogicalOperator, UnaryOperator, UpdateOperator,
    },
    precedence::Precedence,
};

pub fn kind_to_precedence(kind: Kind) -> Option<Precedence> {
    match kind {
        Kind::Pipe2 => Some(Precedence::LogicalOr),
        Kind::Amp2 => Some(Precedence::LogicalAnd),
        Kind::Pipe => Some(Precedence::BitwiseOr),
        Kind::Caret => Some(Precedence::BitwiseXor),
        Kind::Amp => Some(Precedence::BitwiseAnd),
        Kind::Eq2 => Some(Precedence::Equality),
        Kind::Plus | Kind::Minus => Some(Precedence::Add),
        Kind::Star | Kind::Slash | Kind::Percent => Some(Precedence::Multiply),
        Kind::LAngle | Kind::RAngle | Kind::LtEq | Kind::GtEq => Some(Precedence::Relational),
        Kind::Star2 => Some(Precedence::Exponential),
        _ => None,
    }
}

pub fn map_assignment_operator(kind: Kind) -> AssignmentOperator {
    match kind {
        Kind::Eq => AssignmentOperator::Assign,
        Kind::PlusEq => AssignmentOperator::Addition,
        Kind::MinusEq => AssignmentOperator::Subtraction,
        Kind::StarEq => AssignmentOperator::Multiplication,
        Kind::SlashEq => AssignmentOperator::Division,
        Kind::PercentEq => AssignmentOperator::Remainder,
        Kind::Amp2Eq => AssignmentOperator::LogicalAnd,
        Kind::Pipe2Eq => AssignmentOperator::LogicalOr,
        Kind::Star2Eq => AssignmentOperator::Exponential,
        Kind::PipeEq => AssignmentOperator::BitwiseOR,
        Kind::CaretEq => AssignmentOperator::BitwiseXOR,
        Kind::AmpEq => AssignmentOperator::BitwiseAnd,
        _ => unreachable!("Update Operator: {kind:?}"),
    }
}

pub fn map_binary_operator(kind: Kind) -> BinaryOperator {
    match kind {
        Kind::Plus => BinaryOperator::Addition,
        Kind::Eq2 => BinaryOperator::Equality,
        Kind::Minus => BinaryOperator::Subtraction,
        Kind::Star => BinaryOperator::Multiplication,
        Kind::Slash => BinaryOperator::Division,
        Kind::Percent => BinaryOperator::Remainder,
        Kind::Star2 => BinaryOperator::Exponential,
        Kind::Neq => BinaryOperator::Inequality,
        Kind::LAngle => BinaryOperator::LessThan,
        Kind::LtEq => BinaryOperator::LessEqualThan,
        Kind::RAngle => BinaryOperator::GreaterThan,
        Kind::GtEq => BinaryOperator::GreaterEqualThan,
        Kind::Pipe => BinaryOperator::BitwiseOR,
        Kind::Caret => BinaryOperator::BitwiseXOR,
        Kind::Amp => BinaryOperator::BitwiseAnd,
        _ => unreachable!("Binary Operator: {kind:?}"),
    }
}

pub fn map_unary_operator(kind: Kind) -> UnaryOperator {
    match kind {
        Kind::Minus => UnaryOperator::UnaryNegation,
        Kind::Plus => UnaryOperator::UnaryPlus,
        _ => unreachable!("Unary Operator: {kind:?}"),
    }
}

pub fn map_logical_operator(kind: Kind) -> LogicalOperator {
    match kind {
        Kind::Pipe2 => LogicalOperator::Or,
        Kind::Amp2 => LogicalOperator::And,
        _ => unreachable!("Logical Operator: {kind:?}"),
    }
}

pub fn map_update_operator(kind: Kind) -> UpdateOperator {
    match kind {
        Kind::Plus2 => UpdateOperator::Increment,
        Kind::Minus2 => UpdateOperator::Decrement,
        _ => unreachable!("Update Operator: {kind:?}"),
    }
}
