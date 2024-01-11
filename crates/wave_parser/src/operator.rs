use wave_lexer::Kind;
use wave_syntax::operator::AssignmentOperator;

pub fn map_assignment_operator(kind: Kind) -> AssignmentOperator {
    match kind {
        Kind::Eq => AssignmentOperator::Assign,
        _ => unreachable!("Update Operator: {kind:?}"),
    }
}
