use std::cell::RefCell;
use std::rc::Rc;

use crate::evaluator::Primitive;
use crate::Runtime;
use crate::{diagnostics, environment::Environment};
use wave_ast::ast::Expression;
use wave_diagnostics::Result;
use wave_span::GetSpan;
use wave_syntax::operator::{BinaryOperator, LogicalOperator};

impl<'a> Runtime<'a> {
    // Arithmetic operations
    pub fn eval_arithmetic(
        &self,
        left: &Expression<'_>,
        right: &Expression<'_>,
        environment: Rc<RefCell<Environment<'a>>>,
        operator: &BinaryOperator,
    ) -> Result<Primitive<'a>> {
        let left_eval = self.eval_expression(left, Rc::clone(&environment))?;
        let right_eval = self.eval_expression(right, Rc::clone(&environment))?;

        match (left_eval, right_eval) {
            (Primitive::Number(left), Primitive::Number(right)) => match operator {
                BinaryOperator::Addition => Ok(Primitive::Number(left + right)),
                BinaryOperator::Subtraction => Ok(Primitive::Number(left - right)),
                BinaryOperator::Multiplication => Ok(Primitive::Number(left * right)),
                BinaryOperator::Division => Ok(Primitive::Number(left / right)),
                BinaryOperator::Remainder => Ok(Primitive::Number(left % right)),
                BinaryOperator::Exponential => Ok(Primitive::Number(left.powf(right))),
                _ => unreachable!(),
            },
            _ => Err(diagnostics::InvalidNumber(left.span().merge(&right.span())).into()),
        }
    }

    // Ord operations
    pub fn eval_ord(
        &self,
        left: &Expression<'_>,
        right: &Expression<'_>,
        environment: Rc<RefCell<Environment<'a>>>,
        operator: &BinaryOperator,
    ) -> Result<Primitive<'a>> {
        let left_eval = self.eval_expression(left, Rc::clone(&environment))?;
        let right_eval = self.eval_expression(right, Rc::clone(&environment))?;

        match (left_eval, right_eval) {
            (Primitive::Number(l), Primitive::Number(r)) => match operator {
                BinaryOperator::LessThan => Ok(Primitive::Boolean(l < r)),
                BinaryOperator::LessEqualThan => Ok(Primitive::Boolean(l <= r)),
                BinaryOperator::GreaterThan => Ok(Primitive::Boolean(l > r)),
                BinaryOperator::GreaterEqualThan => Ok(Primitive::Boolean(l >= r)),
                BinaryOperator::Equality => Ok(Primitive::Boolean(l == r)),
                BinaryOperator::Inequality => Ok(Primitive::Boolean(l != r)),
                _ => unreachable!(),
            },
            (Primitive::Boolean(l), Primitive::Boolean(r)) => match operator {
                BinaryOperator::Equality => Ok(Primitive::Boolean(l == r)),
                BinaryOperator::Inequality => Ok(Primitive::Boolean(l != r)),
                _ => Err(diagnostics::InvalidNumber(left.span().merge(&right.span())).into()),
            },

            (Primitive::String(l), Primitive::String(r)) => match operator {
                BinaryOperator::Equality => Ok(Primitive::Boolean(l == r)),
                BinaryOperator::Inequality => Ok(Primitive::Boolean(l != r)),
                _ => Err(diagnostics::InvalidNumber(left.span().merge(&right.span())).into()),
            },
            _ => Err(diagnostics::TypeMismatch(left.span().merge(&right.span())).into()),
        }
    }

    // Bitwise operations
    pub fn eval_bitwise(
        &self,
        left: &Expression<'_>,
        right: &Expression<'_>,
        environment: Rc<RefCell<Environment<'a>>>,
        operator: &BinaryOperator,
    ) -> Result<Primitive<'a>> {
        let left_eval = self.eval_expression(left, Rc::clone(&environment))?;
        let right_eval = self.eval_expression(right, Rc::clone(&environment))?;

        match (left_eval, right_eval) {
            (Primitive::Number(left), Primitive::Number(right)) => match operator {
                BinaryOperator::BitwiseOR => {
                    Ok(Primitive::Number((left as u64 | right as u64) as f64))
                }
                BinaryOperator::BitwiseAnd => {
                    Ok(Primitive::Number((left as u64 & right as u64) as f64))
                }
                BinaryOperator::BitwiseXOR => {
                    Ok(Primitive::Number((left as u64 ^ right as u64) as f64))
                }
                _ => unreachable!(),
            },
            _ => Err(diagnostics::InvalidNumber(left.span().merge(&right.span())).into()),
        }
    }

    pub fn eval_logical(
        &self,
        left: &Expression<'_>,
        right: &Expression<'_>,
        environment: Rc<RefCell<Environment<'a>>>,
        operator: &LogicalOperator,
    ) -> Result<Primitive<'a>> {
        let left_eval = self.eval_expression(left, Rc::clone(&environment))?;
        let right_eval = self.eval_expression(right, Rc::clone(&environment))?;

        match (left_eval, right_eval) {
            (Primitive::Boolean(left), Primitive::Boolean(right)) => match operator {
                LogicalOperator::Or => Ok(Primitive::Boolean(left || right)),
                LogicalOperator::And => Ok(Primitive::Boolean(left && right)),
            },
            _ => Err(diagnostics::InvalidBoolean(left.span().merge(&right.span())).into()),
        }
    }
}
