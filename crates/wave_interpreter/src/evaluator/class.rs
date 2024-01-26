use std::cell::RefCell;
use std::ptr;
use std::rc::Rc;

use crate::evaluator::Primitive;
use crate::Runtime;
use crate::{diagnostics, environment::Environment};
use wave_allocator::Box;
use wave_ast::ast::{
    Class, ClassElement, Expression, MemberExpression, NewExpression, PropertyKey,
};
use wave_diagnostics::Result;

impl<'a> Runtime<'a> {
    pub fn eval_class_declaration(
        &self,
        declaration: &Box<'_, Class<'a>>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        unsafe {
            let class = ptr::read(declaration).unbox();
            let class_elements = class.body.unbox().body;

            let env = Rc::new(RefCell::new(Environment::extend(Rc::clone(&environment))));

            for element in class_elements {
                match element {
                    ClassElement::PropertyDefinition(definition) => {
                        let property_name = match &definition.key {
                            PropertyKey::Identifier(identifier) => identifier.name.to_owned(),
                            _ => unreachable!(),
                        };
                        let expr_value = if let Some(expr) = &definition.value {
                            self.eval_expression(expr, Rc::clone(&env))?
                        } else {
                            Primitive::Null
                        };
                        env.borrow_mut().define(property_name, expr_value);
                    }
                    ClassElement::MethodDefinition(definition) => {
                        let method_name = match &definition.key {
                            PropertyKey::Identifier(identifier) => identifier.name.to_owned(),
                            _ => unreachable!(),
                        };
                        let function = self.eval_function(&definition.value, Rc::clone(&env))?;
                        env.borrow_mut().define(method_name, function);
                    }
                }
            }

            if let Some(identifier) = &class.id {
                environment.borrow_mut().define(
                    identifier.name.to_owned(),
                    Primitive::Class(Rc::clone(&env)),
                );
            }

            Ok(Primitive::Class(env))
        }
    }

    pub fn eval_new_expression(
        &self,
        declaration: &Box<'_, NewExpression<'a>>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        match &declaration.callee {
            Expression::Identifier(identifier) => {
                let class_name = identifier.name.to_owned();
                let class = environment.borrow().get(class_name, declaration.span)?;

                match class {
                    Primitive::Class(_) => Ok(class),
                    _ => Err(diagnostics::CannotInstantiateNonClass(identifier.span).into()),
                }
            }
            _ => Err(diagnostics::CannotInstantiateNonClass(declaration.span).into()),
        }
    }

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
                        Primitive::Class(env) => {
                            let property_name = expression.property.name;

                            let property = env.borrow().get(property_name, expression.span)?;

                            match property {
                                Primitive::Function(_, _, _)
                                | Primitive::Number(_)
                                | Primitive::String(_)
                                | Primitive::Boolean(_)
                                | Primitive::Array(_)
                                | Primitive::Null => Ok(property),
                                _ => Err(diagnostics::CannotAccessProperty(expression.span).into()),
                            }
                        }
                        _ => todo!(),
                    }
                }
                MemberExpression::ComputedMemberExpression(_) => {
                    todo!()
                }
            }
        }
    }
}
