use std::cell::RefCell;
use std::rc::Rc;

use crate::environment::Environment;
use crate::evaluator::Primitive;
use crate::Runtime;
use wave_allocator::Box;
use wave_ast::ast::Class;
use wave_diagnostics::Result;

impl<'a> Runtime<'a> {
    pub fn eval_class_declaration(
        &self,
        declaration: &Box<'_, Class>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        Ok(Primitive::Null)
    }
}
