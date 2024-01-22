use std::{cell::RefCell, rc::Rc};

use crate::{diagnostics, evaluator::Primitive};
use rustc_hash::FxHashMap;
use wave_diagnostics::Result;
use wave_span::{Atom, Span};

#[derive(Default, Debug)]
pub struct Environment<'a> {
    pub values: FxHashMap<Atom, Primitive<'a>>,
    // TODO : Build the semantic analyzer first
    // REFACTOR : Repalce this rc to environment with scope tree from the sematic analysis
    // I can probably just slug the variable name and use the scope tree to find the variable
    pub outer: Option<Rc<RefCell<Environment<'a>>>>,
}

impl<'a> Environment<'a> {
    pub fn get(&self, name: Atom, span: Span) -> Result<Primitive<'a>> {
        match self.values.get(&name) {
            Some(v) => Ok(v.clone()),
            None => match &self.outer {
                Some(outer) => outer.as_ref().borrow().get(name, span),
                None => Err(diagnostics::VariableNotFound(span).into()),
            },
        }
    }

    pub fn define(&mut self, name: Atom, value: Primitive<'a>) {
        self.values.insert(name, value);
    }

    pub fn extend(outer: Rc<RefCell<Environment<'a>>>) -> Environment<'a> {
        Environment {
            values: FxHashMap::default(),
            outer: Some(outer),
        }
    }
}
