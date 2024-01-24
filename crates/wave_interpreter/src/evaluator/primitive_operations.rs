use std::cell::RefCell;
use std::rc::Rc;

use crate::evaluator::Primitive;
use crate::Runtime;
use crate::{diagnostics, environment::Environment};
use wave_allocator::Box;
use wave_ast::ast::{
    BinaryExpression, Expression, LogicalExpression, SimpleAssignmentTarget, UnaryExpression,
    UpdateExpression,
};
use wave_diagnostics::Result;
use wave_span::GetSpan;
use wave_syntax::operator::{BinaryOperator, LogicalOperator, UnaryOperator, UpdateOperator};

impl<'a> Runtime<'a> {
    pub fn eval_binary_expression(
        &self,
        expression: &BinaryExpression<'_>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        let left = &expression.left;
        let right = &expression.right;
        match expression.operator {
            BinaryOperator::Addition
            | BinaryOperator::Subtraction
            | BinaryOperator::Multiplication
            | BinaryOperator::Division
            | BinaryOperator::Remainder
            | BinaryOperator::Exponential => {
                self.eval_arithmetic(left, right, environment, &expression.operator)
            }

            BinaryOperator::Equality
            | BinaryOperator::Inequality
            | BinaryOperator::LessThan
            | BinaryOperator::LessEqualThan
            | BinaryOperator::GreaterThan
            | BinaryOperator::GreaterEqualThan => {
                self.eval_ord(left, right, environment, &expression.operator)
            }

            BinaryOperator::BitwiseOR | BinaryOperator::BitwiseAnd | BinaryOperator::BitwiseXOR => {
                self.eval_bitwise(left, right, environment, &expression.operator)
            }
        }
    }

    pub fn eval_logical_expression(
        &self,
        expression: &LogicalExpression<'_>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        let left = &expression.left;
        let right = &expression.right;

        match expression.operator {
            LogicalOperator::Or | LogicalOperator::And => {
                self.eval_logical(left, right, environment, &expression.operator)
            }
        }
    }

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

    pub fn eval_unary_expression(
        &self,
        expression: &Box<'_, UnaryExpression>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        let value = self.eval_expression(&expression.argument, Rc::clone(&environment))?;
        match expression.operator {
            UnaryOperator::UnaryPlus => match value {
                Primitive::Number(value) => Ok(Primitive::Number(value.abs())),
                _ => Err(diagnostics::InvalidNumber(expression.span).into()),
            },
            UnaryOperator::UnaryNegation => match value {
                Primitive::Number(value) => Ok(Primitive::Number(-value)),
                _ => Err(diagnostics::InvalidNumber(expression.span).into()),
            },
            UnaryOperator::LogicalNot => match value {
                Primitive::Boolean(value) => Ok(Primitive::Boolean(!value)),
                _ => Err(diagnostics::InvalidBoolean(expression.span).into()),
            },
        }
    }

    pub fn eval_update_expression(
        &self,
        expression: &Box<'_, UpdateExpression>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        let SimpleAssignmentTarget::AssignmentTargetIdentifier(identifier) = &expression.argument;

        match expression.operator {
            UpdateOperator::Increment => {
                let value = environment
                    .borrow()
                    .get(identifier.name.to_owned(), identifier.span)?;
                match value {
                    Primitive::Number(value) => {
                        let new_value = Primitive::Number(value + 1.0);
                        environment
                            .borrow_mut()
                            .define(identifier.name.to_owned(), new_value.clone());
                        Ok(new_value)
                    }
                    _ => Err(diagnostics::InvalidNumber(expression.span).into()),
                }
            }
            UpdateOperator::Decrement => {
                let value = environment
                    .borrow()
                    .get(identifier.name.to_owned(), identifier.span)?;

                match value {
                    Primitive::Number(value) => {
                        let new_value = Primitive::Number(value + 1.0);
                        environment
                            .borrow_mut()
                            .define(identifier.name.to_owned(), new_value.clone());
                        Ok(new_value)
                    }
                    _ => Err(diagnostics::InvalidNumber(expression.span).into()),
                }
            }
        }
    }
}
