use std::fmt::Debug;
use std::vec::Vec as StdVec;
use std::{cell::RefCell, ptr, rc::Rc};
use wave_allocator::Vec;
use wave_ast::ast::{FormalParameter, Statement};

use crate::environment::Environment;

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
    Instance(Rc<RefCell<Environment<'a>>>),
    This(Rc<RefCell<Environment<'a>>>),
    Return(Box<Primitive<'a>>),
    Break,
    Continue,
    Null,
}

impl<'a> PartialEq for Primitive<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Primitive::Number(a), Primitive::Number(b)) => a == b,
            (Primitive::Boolean(a), Primitive::Boolean(b)) => a == b,
            (Primitive::String(a), Primitive::String(b)) => a == b,
            (Primitive::Array(a), Primitive::Array(b)) => a == b,
            (Primitive::Return(a), Primitive::Return(b)) => a == b,
            _ => false,
        }
    }
}

impl<'a> Debug for Primitive<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Primitive::Number(value) => write!(f, "{}", value),
            Primitive::Boolean(value) => write!(f, "{}", value),
            Primitive::String(value) => write!(f, "{}", value),
            Primitive::Array(value) => write!(f, "{:?}", value),
            Primitive::Function(_, _, _) => write!(f, "Function"),
            Primitive::Class(env) => write!(f, "{:?}", env.borrow().values),
            Primitive::Instance(env) => write!(f, "{:?}", env.borrow().values),
            Primitive::This(env) => write!(f, "{:?}", env.borrow().values),
            Primitive::Return(value) => write!(f, "Return({:?})", value),
            Primitive::Break => write!(f, "Break"),
            Primitive::Continue => write!(f, "Continue"),
            Primitive::Null => write!(f, "Null"),
        }
    }
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
            Primitive::Instance(environment) => Primitive::Instance(Rc::clone(environment)),
            Primitive::This(environment) => Primitive::This(Rc::clone(environment)),
            Primitive::Null => Primitive::Null,
            Primitive::Break => Primitive::Break,
            Primitive::Continue => Primitive::Continue,
            Primitive::Return(value) => Primitive::Return(Box::clone(value)),
        }
    }
}
