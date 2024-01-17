use wave_allocator::{Allocator, Vec};
use wave_ast::{
    ast::{
        ArrayExpression, ArrayExpressionElement, BinaryExpression, Expression, ExpressionStatement,
        LogicalExpression, Program, Statement,
    },
    BooleanLiteral, NumberLiteral, StringLiteral,
};
use wave_diagnostics::Result;
use wave_syntax::operator::{BinaryOperator, LogicalOperator};

use crate::{ArithmeticWrapper, LogicalWrapper, Runtime};

// Eval Return
#[derive(Debug, PartialEq)]
pub enum ER<'a> {
    Number(f64),
    Boolean(bool),
    String(&'a str),
    Array(Vec<'a, ER<'a>>),
    Null,
}

pub trait Eval {
    fn eval(&self) -> Result<ER>;
}

pub trait EvalWithAllocator<'a> {
    fn eval_a(&self, allocator: &'a Allocator) -> Result<ER>;
}

impl Eval for Runtime<'_> {
    fn eval(&self) -> Result<ER> {
        Program::eval_a(&self.program, self.allocator)
    }
}

impl<'a> EvalWithAllocator<'a> for Program<'a> {
    fn eval_a(&self, allocator: &'a Allocator) -> Result<ER> {
        let mut result = ER::Null;
        for statement in &self.body {
            result = Statement::eval_a(statement, allocator)?;
        }
        Ok(result)
    }
}

impl<'a> EvalWithAllocator<'a> for Statement<'a> {
    fn eval_a(&self, allocator: &'a Allocator) -> Result<ER> {
        match self {
            Statement::ExpressionStatement(expression_stmt) => {
                let result = ExpressionStatement::eval_a(expression_stmt, allocator)?;
                Ok(result)
            }
            _ => unimplemented!(),
        }
    }
}

impl<'a> EvalWithAllocator<'a> for ExpressionStatement<'a> {
    fn eval_a(&self, allocator: &'a Allocator) -> Result<ER> {
        self.expression.eval_a(allocator)
    }
}

impl Eval for Expression<'_> {
    fn eval(&self) -> Result<ER> {
        match &self {
            Expression::BooleanLiteral(expression) => Ok(BooleanLiteral::eval(expression)?),
            Expression::NumberLiteral(expression) => Ok(NumberLiteral::eval(expression)?),
            Expression::StringLiteral(expression) => Ok(StringLiteral::eval(expression)?),
            Expression::BinaryExpression(expression) => Ok(BinaryExpression::eval(expression)?),
            Expression::LogicalExpression(expression) => Ok(LogicalExpression::eval(expression)?),
            _ => unimplemented!(),
        }
    }
}

#[rustfmt::skip]
impl<'a> EvalWithAllocator<'a> for Expression<'a> {
    fn eval_a(&self, allocator: &'a Allocator) -> Result<ER> {
        match &self {
            Expression::ArrayExpression(expression) => { Ok(ArrayExpression::eval_a(expression, allocator)?) }
            _ => { self.eval() }
        }
    }
}

#[rustfmt::skip]
impl Eval for BinaryExpression<'_> {
    fn eval(&self) -> Result<ER> {
        let left = &self.left;
        let right = &self.right;

        match self.operator {
            BinaryOperator::Addition => Ok(ER::Number(ArithmeticWrapper(left).add_e(ArithmeticWrapper(right))?)),
            BinaryOperator::Subtraction => Ok(ER::Number(ArithmeticWrapper(left).sub_e(ArithmeticWrapper(right))?)),
            BinaryOperator::Multiplication => Ok(ER::Number(ArithmeticWrapper(left).mul_e(ArithmeticWrapper(right))?)),
            BinaryOperator::Division => Ok(ER::Number(ArithmeticWrapper(left).div_e(ArithmeticWrapper(right))?)),
            BinaryOperator::Remainder => Ok(ER::Number(ArithmeticWrapper(left).rem_e(ArithmeticWrapper(right))?)),
            BinaryOperator::Exponential => Ok(ER::Number(ArithmeticWrapper(left).pow_e(ArithmeticWrapper(right))?)),
            BinaryOperator::BitwiseOR => Ok(ER::Number(ArithmeticWrapper(left).b_or(ArithmeticWrapper(right))?)),
            BinaryOperator::BitwiseAnd => Ok(ER::Number(ArithmeticWrapper(left).b_and(ArithmeticWrapper(right))?)),
            BinaryOperator::BitwiseXOR => Ok(ER::Number(ArithmeticWrapper(left).b_xor(ArithmeticWrapper(right))?)),

            BinaryOperator::LessThan => Ok(ER::Boolean(LogicalWrapper(left).lt(LogicalWrapper(right))?)),
            BinaryOperator::LessEqualThan => Ok(ER::Boolean(LogicalWrapper(left).lt_e(LogicalWrapper(right))?)),
            BinaryOperator::GreaterThan => Ok(ER::Boolean(LogicalWrapper(left).gt(LogicalWrapper(right))?)),
            BinaryOperator::GreaterEqualThan => Ok(ER::Boolean(LogicalWrapper(left).gt_e(LogicalWrapper(right))?)),
            BinaryOperator::Equality => Ok(ER::Boolean(LogicalWrapper(left).eq(LogicalWrapper(right))?)),
            BinaryOperator::Inequality => Ok(ER::Boolean(LogicalWrapper(left).ne(LogicalWrapper(right))?)),
        }
    }
}

#[rustfmt::skip]
impl Eval for LogicalExpression<'_> {
    fn eval(&self) -> Result<ER> {
        let left = &self.left;
        let right = &self.right;

        match self.operator {
            LogicalOperator::Or => Ok(ER::Boolean(LogicalWrapper(left).or(LogicalWrapper(right))?)),
            LogicalOperator::And => Ok(ER::Boolean( LogicalWrapper(left).and(LogicalWrapper(right))?,)),
        }
    }
}

impl<'a> EvalWithAllocator<'a> for ArrayExpression<'a> {
    fn eval_a(&self, allocator: &'a Allocator) -> Result<ER> {
        let mut result = Vec::new_in(allocator);
        for expression in &self.elements {
            result.push(expression.eval_a(allocator)?);
        }
        Ok(ER::Array(result))
    }
}

impl<'a> EvalWithAllocator<'a> for ArrayExpressionElement<'a> {
    fn eval_a(&self, allocator: &'a Allocator) -> Result<ER> {
        match self {
            ArrayExpressionElement::Expression(expression) => Ok(expression.eval_a(allocator)?),
        }
    }
}

impl Eval for BooleanLiteral {
    fn eval(&self) -> Result<ER> {
        Ok(ER::Boolean(self.value))
    }
}

impl Eval for NumberLiteral<'_> {
    fn eval(&self) -> Result<ER> {
        Ok(ER::Number(self.value))
    }
}

impl Eval for StringLiteral {
    fn eval(&self) -> Result<ER> {
        Ok(ER::String(self.value.as_str()))
    }
}
