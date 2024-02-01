use std::cell::RefCell;
use std::ptr;
use std::rc::Rc;

use crate::evaluator::Primitive;
use crate::Runtime;
use crate::{diagnostics, environment::Environment};
use std::vec::Vec as StdVec;
use wave_allocator::Box;
use wave_ast::ast::{
    Argument, Class, ClassElement, Expression, MemberExpression, NewExpression, PropertyKey, Super,
    ThisExpression,
};
use wave_diagnostics::Result;
use wave_span::{Atom, Span};

const CONSTRUCTOR: &str = "this_constructor";

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

            if let Some(identifier) = &class.id {
                for element in class_elements {
                    match element {
                        ClassElement::PropertyDefinition(definition) => {
                            let property_name = match &definition.key {
                                PropertyKey::Identifier(identifier) => identifier.name.to_owned(),
                                _ => unreachable!(),
                            };
                            let property_name = self.bind_this(property_name);
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
                            let method_name = self.bind_this(method_name);
                            let function =
                                self.eval_function(&definition.value, Rc::clone(&env))?;
                            env.borrow_mut().define(method_name, function);
                        }
                    }
                }

                if let Some(Expression::Identifier(ident)) = &declaration.super_class {
                    env.borrow_mut()
                        .define("super".into(), Primitive::String(ident.name.to_string()));
                };

                environment
                    .borrow_mut()
                    .define(identifier.name.to_owned(), Primitive::Class(env));
            }

            Ok(Primitive::Null)
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
                    Primitive::Class(class_env) => {
                        let class_env = Rc::new(RefCell::new(Environment::extend(class_env)));

                        let constuctor = class_env
                            .borrow()
                            .get(CONSTRUCTOR.into(), declaration.span)?;

                        let mut arguments = vec![];
                        for arg in &declaration.arguments {
                            match arg {
                                Argument::Expression(expression) => {
                                    arguments.push(
                                        self.eval_expression(expression, Rc::clone(&class_env))?,
                                    );
                                }
                            }
                        }

                        let Primitive::Class(instance_env) = self.apply_constructor(
                            constuctor,
                            arguments,
                            declaration.span,
                            class_env,
                        )?
                        else {
                            return Err(
                                diagnostics::CannotInstantiateNonClass(identifier.span).into()
                            );
                        };

                        Ok(Primitive::Instance(instance_env))
                    }
                    _ => Err(diagnostics::CannotInstantiateNonClass(identifier.span).into()),
                }
            }
            _ => Err(diagnostics::CannotInstantiateNonClass(declaration.span).into()),
        }
    }

    pub fn apply_constructor(
        &self,
        function: Primitive<'a>,
        arguments: StdVec<Primitive<'a>>,
        callee_span: Span,
        env: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        match function {
            Primitive::Function(params, body, _) => {
                match params {
                    Some(params) => {
                        if params.len() != arguments.len() {
                            return Err(diagnostics::InvalidNumberOfArguments(callee_span).into());
                        }

                        for (param, arg) in params.iter().zip(arguments) {
                            let param_name = self.get_atom_formal_parameters(param);
                            env.borrow_mut().define(param_name, arg);
                        }
                    }
                    None => {
                        if !arguments.is_empty() {
                            return Err(diagnostics::InvalidNumberOfArguments(callee_span).into());
                        }
                    }
                }

                match body {
                    Some(body) => {
                        let eval = self.eval_block(&body, Rc::clone(&env))?;
                        self.unwrap_return_value(eval)?;
                        Ok(Primitive::Class(env))
                    }
                    None => Ok(Primitive::Null),
                }
            }
            _ => Err(diagnostics::CannotCallNonFunction(callee_span).into()),
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
                MemberExpression::ComputedMemberExpression(_) => {
                    todo!()
                }
            }
        }
    }

    pub fn get_parent_class(
        &self,
        span: Span,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        let parent_class = environment.borrow().get("super".into(), span)?;

        let Primitive::String(parent_class_name) = parent_class else {
            unreachable!()
        };

        environment.borrow().get(parent_class_name.into(), span)
    }

    pub fn eval_super_expression(
        &self,
        super_call: &Box<'_, Super>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        let Primitive::Class(parent_class) =
            self.get_parent_class(super_call.span, Rc::clone(&environment))?
        else {
            unreachable!()
        };

        let super_call = parent_class
            .borrow()
            .get(CONSTRUCTOR.into(), super_call.span)?;

        Ok(super_call)
    }

    pub fn eval_this_expression(
        &self,
        _: &ThisExpression,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        Ok(Primitive::This(environment))
    }

    pub fn bind_this(&self, atom: Atom) -> Atom {
        ("this_".to_string() + atom.as_ref()).into()
    }
}
