use std::{cell::RefCell, rc::Rc};

use crate::{diagnostics, eval::eval_result::ER};
use rustc_hash::FxHashMap;
use wave_diagnostics::Result;
use wave_span::{Atom, Span};

#[derive(Default, Debug)]
pub struct Environment<'a> {
    pub values: FxHashMap<Atom, ER<'a>>,
    pub outer: Option<Rc<RefCell<Environment<'a>>>>,
}

impl<'a> Environment<'a> {
    pub fn get(&self, name: Atom, span: Span) -> Result<ER<'a>> {
        match self.values.get(&name) {
            Some(v) => Ok(v.clone()),
            None => match &self.outer {
                Some(outer) => outer.as_ref().borrow().get(name, span),
                None => Err(diagnostics::VariableNotFound(span).into()),
            },
        }
    }

    pub fn define(&mut self, name: Atom, value: ER<'a>) {
        self.values.insert(name, value);
    }

    pub fn extend(outer: Rc<RefCell<Environment<'a>>>) -> Environment<'a> {
        Environment {
            values: FxHashMap::default(),
            outer: Some(outer),
        }
    }
}
