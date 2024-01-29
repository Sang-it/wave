use std::cell::RefCell;
use std::ptr;
use std::rc::Rc;

use crate::evaluator::Primitive;
use crate::Runtime;
use crate::{diagnostics, environment::Environment};
use wave_allocator::Box;
use wave_ast::ast::{
    AssignmentExpression, AssignmentTarget, MemberExpression, SimpleAssignmentTarget,
};
use wave_diagnostics::Result;
use wave_syntax::operator::AssignmentOperator;

impl<'a> Runtime<'a> {
    pub fn eval_assignment_expression(
        &self,
        expression: &Box<'_, AssignmentExpression<'a>>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        let operator = &expression.operator;

        match operator {
            AssignmentOperator::Assign => self.eval_reassignment(expression, environment),

            AssignmentOperator::Addition
            | AssignmentOperator::Subtraction
            | AssignmentOperator::Multiplication
            | AssignmentOperator::Division
            | AssignmentOperator::Remainder
            | AssignmentOperator::Exponential => {
                self.eval_arithmetic_assignment(expression, environment)
            }

            AssignmentOperator::BitwiseOR
            | AssignmentOperator::BitwiseXOR
            | AssignmentOperator::BitwiseAnd => {
                self.eval_bitwise_assignment(expression, environment)
            }

            AssignmentOperator::LogicalOr | AssignmentOperator::LogicalAnd => {
                self.eval_logical_assignment(expression, environment)
            }
        }
    }

    pub fn eval_reassignment(
        &self,
        expression: &Box<'_, AssignmentExpression<'a>>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        let right_eval = self.eval_expression(&expression.right, Rc::clone(&environment))?;

        match &expression.left {
            AssignmentTarget::SimpleAssignmentTarget(target) => match target {
                SimpleAssignmentTarget::AssignmentTargetIdentifier(identifier) => {
                    environment
                        .borrow_mut()
                        .define(identifier.name.to_owned(), right_eval);
                }
                SimpleAssignmentTarget::MemberAssignmentTarget(member_expression) => unsafe {
                    let member_expression = ptr::read(member_expression).unbox();
                    match member_expression {
                        MemberExpression::StaticMemberExpression(static_member) => {
                            let env = self
                                .eval_expression(&static_member.object, Rc::clone(&environment))?;

                            let property_name = self.bind_this(static_member.property.name);

                            match env {
                                Primitive::This(this_env) => {
                                    this_env.borrow_mut().define(property_name, right_eval);
                                }
                                _ => todo!(),
                            }
                        }
                        _ => todo!(),
                    }
                },
            },
        };
        Ok(Primitive::Null)
    }

    pub fn eval_arithmetic_assignment(
        &self,
        expression: &Box<'_, AssignmentExpression<'a>>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        let left_identifier = match &expression.left {
            AssignmentTarget::SimpleAssignmentTarget(target) => match target {
                SimpleAssignmentTarget::AssignmentTargetIdentifier(identifier) => identifier,
                _ => unreachable!(),
            },
        };
        let left_current = environment
            .borrow()
            .get(left_identifier.name.to_owned(), left_identifier.span)?;
        let right_eval = self.eval_expression(&expression.right, Rc::clone(&environment))?;
        match (left_current, right_eval) {
            (Primitive::Number(l), Primitive::Number(r)) => match &expression.operator {
                AssignmentOperator::Addition => {
                    environment
                        .borrow_mut()
                        .define(left_identifier.name.to_owned(), Primitive::Number(l + r));
                    Ok(Primitive::Null)
                }
                AssignmentOperator::Subtraction => {
                    environment
                        .borrow_mut()
                        .define(left_identifier.name.to_owned(), Primitive::Number(l - r));
                    Ok(Primitive::Null)
                }
                AssignmentOperator::Multiplication => {
                    environment
                        .borrow_mut()
                        .define(left_identifier.name.to_owned(), Primitive::Number(l * r));
                    Ok(Primitive::Null)
                }
                AssignmentOperator::Division => {
                    environment
                        .borrow_mut()
                        .define(left_identifier.name.to_owned(), Primitive::Number(l / r));
                    Ok(Primitive::Null)
                }
                AssignmentOperator::Remainder => {
                    environment
                        .borrow_mut()
                        .define(left_identifier.name.to_owned(), Primitive::Number(l % r));
                    Ok(Primitive::Null)
                }
                AssignmentOperator::Exponential => {
                    environment.borrow_mut().define(
                        left_identifier.name.to_owned(),
                        Primitive::Number(l.powf(r)),
                    );
                    Ok(Primitive::Null)
                }
                _ => unreachable!(),
            },
            _ => Err(diagnostics::InvalidNumber(expression.span).into()),
        }
    }

    pub fn eval_bitwise_assignment(
        &self,
        expression: &Box<'_, AssignmentExpression<'a>>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        let left_identifier = match &expression.left {
            AssignmentTarget::SimpleAssignmentTarget(target) => match target {
                SimpleAssignmentTarget::AssignmentTargetIdentifier(identifier) => identifier,
                _ => unreachable!(),
            },
        };
        let left_current = environment
            .borrow()
            .get(left_identifier.name.to_owned(), left_identifier.span)?;
        let right_eval = self.eval_expression(&expression.right, Rc::clone(&environment))?;
        match (left_current, right_eval) {
            (Primitive::Number(l), Primitive::Number(r)) => match &expression.operator {
                AssignmentOperator::BitwiseOR => {
                    environment.borrow_mut().define(
                        left_identifier.name.to_owned(),
                        Primitive::Number((l as i64 | r as i64) as f64),
                    );
                    Ok(Primitive::Null)
                }
                AssignmentOperator::BitwiseAnd => {
                    environment.borrow_mut().define(
                        left_identifier.name.to_owned(),
                        Primitive::Number((l as i64 & r as i64) as f64),
                    );
                    Ok(Primitive::Null)
                }
                AssignmentOperator::BitwiseXOR => {
                    environment.borrow_mut().define(
                        left_identifier.name.to_owned(),
                        Primitive::Number((l as i64 ^ r as i64) as f64),
                    );
                    Ok(Primitive::Null)
                }
                _ => unreachable!(),
            },
            _ => Err(diagnostics::InvalidNumber(expression.span).into()),
        }
    }

    pub fn eval_logical_assignment(
        &self,
        expression: &Box<'_, AssignmentExpression<'a>>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        let left_identifier = match &expression.left {
            AssignmentTarget::SimpleAssignmentTarget(target) => match target {
                SimpleAssignmentTarget::AssignmentTargetIdentifier(identifier) => identifier,
                _ => unreachable!(),
            },
        };

        let left_current = environment
            .borrow()
            .get(left_identifier.name.to_owned(), left_identifier.span)?;

        let right_eval = self.eval_expression(&expression.right, Rc::clone(&environment))?;
        match (left_current, right_eval) {
            (Primitive::Boolean(l), Primitive::Boolean(r)) => match &expression.operator {
                AssignmentOperator::LogicalOr => {
                    environment
                        .borrow_mut()
                        .define(left_identifier.name.to_owned(), Primitive::Boolean(l || r));
                    Ok(Primitive::Null)
                }
                AssignmentOperator::LogicalAnd => {
                    environment
                        .borrow_mut()
                        .define(left_identifier.name.to_owned(), Primitive::Boolean(l && r));
                    Ok(Primitive::Null)
                }
                _ => unreachable!(),
            },
            _ => Err(diagnostics::InvalidBoolean(expression.span).into()),
        }
    }
}
