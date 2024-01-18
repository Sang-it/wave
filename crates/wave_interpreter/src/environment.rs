use rustc_hash::FxHashMap;
use wave_span::Atom;

use crate::eval::ER;

#[derive(Default, Debug)]
pub struct Environment {
    pub values: FxHashMap<Atom, ER>,
}

impl Environment {
    pub fn define(&mut self, name: Atom, value: ER) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Atom) -> Option<&ER> {
        self.values.get(&name)
    }
}
