use std::cell::RefCell;
use std::rc::Rc;

use crate::environment::Environment;
use crate::evaluator::Primitive;
use crate::Runtime;
use std::vec::Vec as StdVec;
use wave_allocator::Box;
use wave_ast::ast::{
    ArrayExpression, ArrayExpressionElement, BinaryExpression, Expression, IdentifierReference,
    LogicalExpression,
};
use wave_diagnostics::Result;
use wave_syntax::operator::{BinaryOperator, LogicalOperator};

impl<'a> Runtime<'a> {
    pub fn eval_expression(
        &self,
        expression: &Expression<'_>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        match expression {
            Expression::BooleanLiteral(expression) => self.eval_boolean_literal(expression),
            Expression::NumberLiteral(expression) => self.eval_number_literal(expression),
            Expression::StringLiteral(expression) => self.eval_string_literal(expression),
            Expression::Identifier(expression) => self.eval_identifier(expression, environment),
            Expression::ArrayExpression(expression) => {
                self.eval_array_expression(expression, environment)
            }
            Expression::BinaryExpression(expression) => {
                self.eval_binary_expression(expression, environment)
            }
            Expression::LogicalExpression(expression) => {
                self.eval_logical_expression(expression, environment)
            }
            Expression::CallExpression(expression) => {
                self.eval_call_expression(expression, environment)
            }
            _ => unimplemented!(),
        }
    }

    fn eval_identifier(
        &self,
        expression: &Box<'_, IdentifierReference>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        environment
            .borrow()
            .get(expression.name.to_owned(), expression.span)
            .map(|v| v.clone())
    }

    fn eval_array_expression(
        &self,
        expression: &Box<'_, ArrayExpression>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        let mut result = StdVec::new();
        for element in &expression.elements {
            let value = self.eval_array_expression_element(element, Rc::clone(&environment))?;
            result.push(value);
        }
        Ok(Primitive::Array(result))
    }

    fn eval_array_expression_element(
        &self,
        expression: &ArrayExpressionElement<'_>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        match expression {
            ArrayExpressionElement::Expression(expression) => {
                self.eval_expression(expression, Rc::clone(&environment))
            }
        }
    }

    fn eval_binary_expression(
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
}
