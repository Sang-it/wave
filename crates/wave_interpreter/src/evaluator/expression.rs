use std::cell::RefCell;
use std::rc::Rc;

use crate::environment::Environment;
use crate::evaluator::Primitive;
use crate::Runtime;
use std::vec::Vec as StdVec;
use wave_allocator::Box;
use wave_ast::ast::{
    ArrayExpression, ArrayExpressionElement, Expression, IdentifierReference,
    ParenthesizedExpression, SequenceExpression,
};
use wave_diagnostics::Result;

impl<'a> Runtime<'a> {
    pub fn eval_expression(
        &self,
        expression: &Expression<'a>,
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
            Expression::AssignmentExpression(expression) => {
                self.eval_assignment_expression(expression, environment)
            }
            Expression::NullLiteral(_) => Ok(Primitive::Null),
            Expression::UnaryExpression(expression) => {
                self.eval_unary_expression(expression, environment)
            }
            Expression::SequenceExpression(expression) => {
                self.eval_sequence_expression(expression, environment)
            }
            Expression::UpdateExpression(expression) => {
                self.eval_update_expression(expression, environment)
            }
            Expression::ParenthesizedExpression(expression) => {
                self.eval_parenthesized_expression(expression, environment)
            }
            Expression::FunctionExpression(expression) => {
                self.eval_function_expression(expression, environment)
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
        expression: &Box<'_, ArrayExpression<'a>>,
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
        expression: &ArrayExpressionElement<'a>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        match expression {
            ArrayExpressionElement::Expression(expression) => {
                self.eval_expression(expression, Rc::clone(&environment))
            }
        }
    }

    fn eval_sequence_expression(
        &self,
        expression: &Box<'_, SequenceExpression<'a>>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        let mut result = Primitive::Null;
        for expression in &expression.expressions {
            result = self.eval_expression(expression, Rc::clone(&environment))?;
        }
        Ok(result)
    }

    fn eval_parenthesized_expression(
        &self,
        expression: &Box<'_, ParenthesizedExpression<'a>>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        self.eval_expression(&expression.expression, environment)
    }
}
