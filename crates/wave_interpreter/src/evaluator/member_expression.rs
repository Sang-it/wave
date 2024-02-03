use std::{cell::RefCell, ptr, rc::Rc};

use wave_ast::ast::MemberExpression;

use super::Primitive;
use crate::{diagnostics, environment::Environment, Runtime};
use wave_allocator::Box;
use wave_diagnostics::Result;

impl<'a> Runtime<'a> {
    pub fn eval_member_expression(
        &self,
        expression: &Box<'_, MemberExpression<'a>>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        unsafe {
            let expression = ptr::read(expression).unbox();

            match expression {
                MemberExpression::StaticMemberExpression(expression) => {
                    let primitive =
                        self.eval_expression(&expression.object, Rc::clone(&environment))?;

                    match primitive {
                        Primitive::Instance(env) | Primitive::This(env) => {
                            let property_name = expression.property.name;
                            let property_name = self.bind_this(property_name);
                            let property = env.borrow().get(property_name.clone(), expression.span);
                            let property = match property {
                                Ok(property) => property,
                                Err(_) => {
                                    let parent =
                                        self.get_parent_class(expression.span, Rc::clone(&env))?;

                                    match parent {
                                        Primitive::Class(parent_class) => parent_class
                                            .borrow()
                                            .get(property_name, expression.span)?,
                                        _ => unreachable!(),
                                    }
                                }
                            };

                            match property {
                                Primitive::Number(_)
                                | Primitive::String(_)
                                | Primitive::Boolean(_)
                                | Primitive::Array(_)
                                | Primitive::Null => Ok(property),
                                Primitive::Function(params, body, _) => {
                                    Ok(Primitive::Function(params, body, Rc::clone(&env)))
                                }
                                _ => Err(diagnostics::CannotAccessProperty(expression.span).into()),
                            }
                        }
                        _ => todo!(),
                    }
                }
                MemberExpression::ComputedMemberExpression(computed_expression) => {
                    let array =
                        self.eval_expression(&computed_expression.object, Rc::clone(&environment))?;
                    let index = self.eval_expression(
                        &computed_expression.expression,
                        Rc::clone(&environment),
                    )?;

                    match (array, index) {
                        (Primitive::Array(array), Primitive::Number(index)) => {
                            let index = index as usize;
                            if index >= array.len() {
                                return Err(diagnostics::IndexOutOfBounds(
                                    computed_expression.span,
                                )
                                .into());
                            }
                            Ok(array[index].clone())
                        }
                        _ => Err(diagnostics::InvalidArrayAccess(computed_expression.span).into()),
                    }
                }
            }
        }
    }
}
