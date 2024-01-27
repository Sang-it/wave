use crate::{diagnostics, Parser};
use wave_ast::ast::{AssignmentTarget, Expression, SimpleAssignmentTarget};
use wave_diagnostics::Result;
use wave_span::GetSpan;

pub trait CoverGrammar<'a, T>: Sized {
    fn cover(value: T, p: &mut Parser<'a>) -> Result<Self>;
}

impl<'a> CoverGrammar<'a, Expression<'a>> for AssignmentTarget<'a> {
    fn cover(expr: Expression<'a>, p: &mut Parser<'a>) -> Result<Self> {
        SimpleAssignmentTarget::cover(expr, p).map(AssignmentTarget::SimpleAssignmentTarget)
    }
}

impl<'a> CoverGrammar<'a, Expression<'a>> for SimpleAssignmentTarget<'a> {
    #[allow(clippy::only_used_in_recursion)]
    fn cover(expr: Expression<'a>, _p: &mut Parser<'a>) -> Result<Self> {
        match expr {
            Expression::Identifier(ident) => {
                Ok(SimpleAssignmentTarget::AssignmentTargetIdentifier(ident))
            }
            Expression::MemberExpression(expr) => {
                Ok(SimpleAssignmentTarget::MemberAssignmentTarget(expr))
            }
            expr => Err(diagnostics::InvalidAssignment(expr.span()).into()),
        }
    }
}
