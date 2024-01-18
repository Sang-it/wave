use wave_allocator::Allocator;
use wave_ast::ast::Program;

pub struct Runtime<'a> {
    pub arena: &'a Allocator,
    pub program: Program<'a>,
}

impl<'a> Runtime<'a> {
    pub fn new(program: Program<'a>, arena: &'a Allocator) -> Self {
        Self { program, arena }
    }
}
