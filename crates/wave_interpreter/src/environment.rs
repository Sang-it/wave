use rustc_hash::FxHashMap;
use wave_allocator::Box;

#[derive(Default)]
pub struct Environment<'a> {
    pub values: FxHashMap<&'a str, Box<'a, f64>>,
}
