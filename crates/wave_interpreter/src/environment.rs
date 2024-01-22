use crate::eval::ER;
use rustc_hash::FxHashMap;
use wave_allocator::Box;
use wave_diagnostics::Result;
use wave_span::{Atom, Span};

#[derive(Default, Debug, Clone)]
pub struct Environment<'a> {
    pub values: FxHashMap<Atom, ER<'a>>,
    pub outer: Option<&'a Box<'a, Environment<'a>>>,
}

impl<'a> Environment<'a> {
    pub fn define(&mut self, name: Atom, value: ER<'a>) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Atom, span: Span) -> Result<&ER<'a>> {
        match self.values.get(&name) {
            Some(v) => Ok(v),
            None => Err(crate::diagnostics::VariableNotFound(span).into()),
        }
    }

    pub fn extend(&'a mut self, environment: &'a Box<'a, Environment<'a>>) {
        self.values = FxHashMap::default();
        self.outer = Some(environment);
    }
}
