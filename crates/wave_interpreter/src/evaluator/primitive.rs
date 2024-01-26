use std::vec::Vec as StdVec;
use std::{cell::RefCell, ptr, rc::Rc};
use wave_allocator::Vec;
use wave_ast::ast::{FormalParameter, Statement};

use crate::environment::Environment;

#[derive(Debug)]
pub enum Primitive<'a> {
    Number(f64),
    Boolean(bool),
    String(String),
    Array(StdVec<Primitive<'a>>),
    Function(
        Option<Vec<'a, FormalParameter<'a>>>,
        Option<Vec<'a, Statement<'a>>>,
        Rc<RefCell<Environment<'a>>>,
    ),
    Class(Rc<RefCell<Environment<'a>>>),
    Return(Box<Primitive<'a>>),
    Break,
    Continue,
    Null,
}

impl<'a> Clone for Primitive<'a> {
    fn clone(&self) -> Self {
        match self {
            Primitive::Number(value) => Primitive::Number(*value),
            Primitive::Boolean(value) => Primitive::Boolean(*value),
            Primitive::String(value) => Primitive::String(value.to_owned()),
            Primitive::Array(value) => Primitive::Array(value.to_owned()),
            Primitive::Function(paramters, function_body, environment) => unsafe {
                let params = ptr::read(paramters);
                let body = ptr::read(function_body);
                Primitive::Function(params, body, Rc::clone(environment))
            },
            Primitive::Class(environment) => Primitive::Class(Rc::clone(environment)),
            Primitive::Null => Primitive::Null,
            Primitive::Break => Primitive::Break,
            Primitive::Continue => Primitive::Continue,
            Primitive::Return(value) => Primitive::Return(Box::clone(value)),
        }
    }
}
