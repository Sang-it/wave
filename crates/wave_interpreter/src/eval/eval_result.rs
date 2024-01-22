use std::vec::Vec as StdVec;
use std::{cell::RefCell, ptr, rc::Rc};
use wave_allocator::Vec;
use wave_ast::ast::{FormalParameter, Statement};

use crate::environment::Environment;

#[derive(Debug)]
pub enum ER<'a> {
    Number(f64),
    Boolean(bool),
    String(String),
    Array(StdVec<ER<'a>>),
    Function(
        Vec<'a, FormalParameter<'a>>,
        Vec<'a, Statement<'a>>,
        Rc<RefCell<Environment<'a>>>,
    ),
    Null,
}

impl<'a> Clone for ER<'a> {
    fn clone(&self) -> Self {
        match self {
            ER::Number(value) => ER::Number(*value),
            ER::Boolean(value) => ER::Boolean(*value),
            ER::String(value) => ER::String(value.to_owned()),
            ER::Array(value) => ER::Array(value.to_owned()),
            ER::Function(paramters, function_body, environment) => unsafe {
                let params = ptr::read(paramters);
                let body = ptr::read(function_body);
                ER::Function(params, body, Rc::clone(environment))
            },
            ER::Null => ER::Null,
        }
    }
}
