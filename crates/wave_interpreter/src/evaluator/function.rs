use std::rc::Rc;
use std::{cell::RefCell, ptr};

use crate::environment::Environment;
use crate::evaluator::Primitive;
use crate::{diagnostics, Runtime};
use std::vec::Vec as StdVec;
use wave_allocator::Box;
use wave_ast::ast::{
    Argument, BindingPatternKind, CallExpression, Expression, FormalParameter, Function,
};
use wave_diagnostics::Result;
use wave_span::{Atom, Span};

#[derive(Clone)]
pub struct InbuiltFunction {
    pub name: Atom,
    pub function: fn(&StdVec<Primitive<'_>>) -> Primitive<'static>,
}

impl<'a> Runtime<'a> {
    pub fn eval_function(
        &self,
        expression: &Box<'_, Function<'a>>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        unsafe {
            let function = ptr::read(expression).unbox();
            let params = function.params.unbox().items;

            match (function.id, function.body) {
                (Some(id), Some(body)) => {
                    let function_name = id.name.to_owned();
                    if self.is_inbuilt_function(&function_name) {
                        return Err(diagnostics::CannotRedeclareInbuiltFunction(id.span).into());
                    }
                    let body = body.unbox().statements;
                    let function =
                        Primitive::Function(Some(params), Some(body), Rc::clone(&environment));
                    environment
                        .borrow_mut()
                        .define(function_name, function.clone());
                    Ok(function)
                }
                (None, Some(body)) => {
                    let body = body.unbox().statements;
                    let function =
                        Primitive::Function(Some(params), Some(body), Rc::clone(&environment));
                    Ok(function)
                }
                (Some(id), None) => {
                    let function_name = id.name.to_owned();
                    if self.is_inbuilt_function(&function_name) {
                        return Err(diagnostics::CannotRedeclareInbuiltFunction(id.span).into());
                    }
                    let function = Primitive::Function(Some(params), None, Rc::clone(&environment));
                    environment
                        .borrow_mut()
                        .define(function_name, function.clone());

                    Ok(function)
                }
                (None, None) => Ok(Primitive::Function(
                    Some(params),
                    None,
                    Rc::clone(&environment),
                )),
            }
        }
    }

    pub fn eval_call_expression(
        &self,
        expression: &Box<'_, CallExpression<'a>>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        match &expression.callee {
            Expression::Identifier(identifier) => {
                let function_name = identifier.name.to_owned();

                let mut arguments = vec![];

                for arg in &expression.arguments {
                    match arg {
                        Argument::Expression(expression) => {
                            arguments
                                .push(self.eval_expression(expression, Rc::clone(&environment))?);
                        }
                    }
                }

                if self.is_inbuilt_function(&function_name) {
                    let in_built = self.get_in_built_function(&function_name);
                    let function = in_built.function;
                    function(&arguments);
                    Ok(Primitive::Null)
                } else {
                    let function = environment.borrow().get(function_name, identifier.span)?;
                    self.apply_function(function, arguments, expression.span)
                }
            }
            Expression::MemberExpression(_) => {
                let function = self.eval_expression(&expression.callee, Rc::clone(&environment))?;

                let mut arguments = vec![];
                for arg in &expression.arguments {
                    match arg {
                        Argument::Expression(expression) => {
                            arguments
                                .push(self.eval_expression(expression, Rc::clone(&environment))?);
                        }
                    }
                }

                self.apply_function(function, arguments, expression.span)
            }
            Expression::FunctionExpression(function) => {
                let function = self.eval_function(function, Rc::clone(&environment))?;

                let mut arguments = vec![];
                for arg in &expression.arguments {
                    match arg {
                        Argument::Expression(expression) => {
                            arguments
                                .push(self.eval_expression(expression, Rc::clone(&environment))?);
                        }
                    }
                }

                self.apply_function(function, arguments, expression.span)
            }
            _ => unreachable!(),
        }
    }

    pub fn apply_function(
        &self,
        function: Primitive<'a>,
        arguments: StdVec<Primitive<'a>>,
        callee_span: Span,
    ) -> Result<Primitive<'a>> {
        match function {
            Primitive::Function(params, body, env) => {
                let env = Rc::new(RefCell::new(Environment::extend(env)));

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
                        let eval = self.eval_block(&body, env)?;
                        self.unwrap_return_value(eval)
                    }
                    None => Ok(Primitive::Null),
                }
            }
            _ => Err(diagnostics::CannotCallNonFunction(callee_span).into()),
        }
    }

    pub fn get_atom_formal_parameters(&self, param: &FormalParameter) -> Atom {
        match &param.pattern.kind {
            BindingPatternKind::BindingIdentifier(identifier) => identifier.name.to_owned(),
        }
    }

    pub fn unwrap_return_value(&self, primitive: Primitive<'a>) -> Result<Primitive<'a>> {
        match primitive {
            Primitive::Return(value) => Ok(*value),
            _ => Ok(primitive),
        }
    }

    pub fn is_inbuilt_function(&self, name: &Atom) -> bool {
        self.inbuilt_functions
            .iter()
            .map(|f| &f.name)
            .collect::<StdVec<_>>()
            .contains(&name)
    }

    pub fn get_in_built_function(&self, name: &Atom) -> InbuiltFunction {
        self.inbuilt_functions
            .iter()
            .find(|f| f.name == *name)
            .expect("Inbuilt function not found.")
            .clone()
    }
}
