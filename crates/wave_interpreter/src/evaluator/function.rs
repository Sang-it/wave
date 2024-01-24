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

impl<'a> Runtime<'a> {
    pub fn eval_function_declaration(
        &self,
        declaration: &Box<'_, Function<'a>>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        unsafe {
            let function = ptr::read(declaration).unbox();
            if let Some(id) = function.id {
                if let Some(body) = function.body {
                    let params = function.params.unbox().items;
                    let body = body.unbox().statements;
                    let env = Rc::clone(&environment);
                    let function = Primitive::Function(params, body, env);
                    environment
                        .borrow_mut()
                        .define(id.name.to_owned(), function);
                }
            }
        }
        Ok(Primitive::Null)
    }

    pub fn eval_call_expression(
        &self,
        expression: &Box<'_, CallExpression>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        match &expression.callee {
            Expression::Identifier(identifier) => {
                let function = environment
                    .borrow()
                    .get(identifier.name.to_owned(), identifier.span)?;
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

    fn apply_function(
        &self,
        function: Primitive<'a>,
        arguments: StdVec<Primitive<'a>>,
        callee_span: Span,
    ) -> Result<Primitive<'a>> {
        match function {
            Primitive::Function(params, body, env) => {
                let env = Rc::new(RefCell::new(Environment::extend(env)));
                if params.len() != arguments.len() {
                    Err(diagnostics::InvalidNumberOfArguments(callee_span).into())
                } else {
                    for (param, arg) in params.iter().zip(arguments) {
                        let param_name = self.get_atom_formal_parameters(param);
                        env.borrow_mut().define(param_name, arg);
                    }
                    let eval = self.eval_block(&body, env)?;
                    self.unwrap_return_value(eval)
                }
            }
            _ => unreachable!(),
        }
    }

    fn get_atom_formal_parameters(&self, param: &FormalParameter) -> Atom {
        match &param.pattern.kind {
            BindingPatternKind::BindingIdentifier(identifier) => identifier.name.to_owned(),
        }
    }

    fn unwrap_return_value(&self, primitive: Primitive<'a>) -> Result<Primitive<'a>> {
        match primitive {
            Primitive::Return(value) => Ok(*value),
            _ => Ok(primitive),
        }
    }
}
