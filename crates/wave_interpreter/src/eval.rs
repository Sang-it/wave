use wave_ast::{
    ast::{BinaryExpression, Expression, ExpressionStatement, Program, Statement},
    BooleanLiteral, NumberLiteral, StringLiteral,
};
use wave_diagnostics::Result;
use wave_syntax::operator::BinaryOperator;

use crate::{ArithmeticWrapper, LogicalWrapper, Runtime};

// Eval Return
#[derive(Debug, PartialEq)]
pub enum ER<'a> {
    Number(f64),
    Boolean(bool),
    String(&'a str),
    Null,
}

pub trait Eval {
    fn eval(&self) -> Result<ER>;
}

impl Eval for Runtime<'_> {
    fn eval(&self) -> Result<ER> {
        Program::eval(&self.program)
    }
}

impl Eval for Program<'_> {
    fn eval(&self) -> Result<ER> {
        let mut result = ER::Null;
        for statement in &self.body {
            result = Statement::eval(statement)?;
        }
        Ok(result)
    }
}

impl Eval for Statement<'_> {
    fn eval(&self) -> Result<ER> {
        match self {
            Statement::ExpressionStatement(expression_stmt) => {
                let result = ExpressionStatement::eval(expression_stmt)?;
                Ok(result)
            }

            _ => unimplemented!(),
        }
    }
}

impl Eval for ExpressionStatement<'_> {
    fn eval(&self) -> Result<ER> {
        self.expression.eval()
    }
}

impl Eval for Expression<'_> {
    fn eval(&self) -> Result<ER> {
        match &self {
            Expression::BooleanLiteral(expression) => Ok(BooleanLiteral::eval(expression)?),
            Expression::NumberLiteral(expression) => Ok(NumberLiteral::eval(expression)?),
            Expression::StringLiteral(expression) => Ok(StringLiteral::eval(expression)?),
            Expression::BinaryExpression(expression) => Ok(BinaryExpression::eval(expression)?),
            _ => unimplemented!(),
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

#[rustfmt::skip]
impl Eval for BinaryExpression<'_> {
    fn eval(&self) -> Result<ER> {
        let left = &self.left;
        let right = &self.right;

        match self.operator {
            BinaryOperator::Addition => Ok(ER::Number( ArithmeticWrapper(left).add_e(ArithmeticWrapper(right))?)),
            BinaryOperator::Subtraction => Ok(ER::Number( ArithmeticWrapper(left).sub_e(ArithmeticWrapper(right))?)),
            BinaryOperator::Multiplication => Ok(ER::Number( ArithmeticWrapper(left).mul_e(ArithmeticWrapper(right))?)),
            BinaryOperator::Division => Ok(ER::Number( ArithmeticWrapper(left).div_e(ArithmeticWrapper(right))?)),
            BinaryOperator::Remainder => Ok(ER::Number( ArithmeticWrapper(left).rem_e(ArithmeticWrapper(right))?)),
            BinaryOperator::Exponential => Ok(ER::Number( ArithmeticWrapper(left).pow_e(ArithmeticWrapper(right))?)),
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
