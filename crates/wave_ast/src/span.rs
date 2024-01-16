use wave_span::{GetSpan, Span};

use crate::ast::{Declaration, Expression, MemberExpression, Statement};

impl<'a> GetSpan for Statement<'a> {
    fn span(&self) -> Span {
        match self {
            Self::BlockStatement(stmt) => stmt.span,
            Self::BreakStatement(stmt) => stmt.span,
            Self::ContinueStatement(stmt) => stmt.span,
            Self::ExpressionStatement(stmt) => stmt.span,
            Self::IfStatement(stmt) => stmt.span,
            Self::ReturnStatement(stmt) => stmt.span,
            Self::WhileStatement(stmt) => stmt.span,
            Self::Declaration(decl) => decl.span(),
        }
    }
}

impl<'a> GetSpan for Expression<'a> {
    fn span(&self) -> Span {
        match self {
            Self::BooleanLiteral(e) => e.span,
            Self::NullLiteral(e) => e.span,
            Self::NumberLiteral(e) => e.span,
            Self::StringLiteral(e) => e.span,
            Self::Identifier(e) => e.span,
            Self::AssignmentExpression(e) => e.span,
            Self::BinaryExpression(e) => e.span,
            Self::SequenceExpression(e) => e.span,
            Self::ParenthesizedExpression(e) => e.span,
            Self::ArrayExpression(e) => e.span,
            Self::CallExpression(e) => e.span,
            Self::UnaryExpression(e) => e.span,
            Self::UpdateExpression(e) => e.span,
            Self::LogicalExpression(e) => e.span,
            Self::ThisExpression(e) => e.span,
            Self::Super(e) => e.span,
            Self::NewExpression(e) => e.span,
            Self::MemberExpression(e) => e.span(),
        }
    }
}

impl<'a> GetSpan for Declaration<'a> {
    fn span(&self) -> Span {
        match self {
            Self::VariableDeclaration(decl) => decl.span,
            Self::FunctionDeclaration(decl) => decl.span,
        }
    }
}

impl<'a> GetSpan for MemberExpression<'a> {
    fn span(&self) -> Span {
        match self {
            Self::ComputedMemberExpression(expr) => expr.span,
            Self::StaticMemberExpression(expr) => expr.span,
        }
    }
}
