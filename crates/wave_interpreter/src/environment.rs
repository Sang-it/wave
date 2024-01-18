use rustc_hash::FxHashMap;
use wave_span::Atom;

use crate::eval::ER;

#[derive(Default)]
pub struct Environment<'a> {
    pub values: FxHashMap<Atom, ER<'a>>,
}

impl<'a> Environment<'a> {
    pub fn define(&mut self, name: Atom, value: ER<'a>) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Atom) -> Option<&ER<'a>> {
        self.values.get(&name)
    }
}
