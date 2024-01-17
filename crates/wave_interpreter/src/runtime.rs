use crate::environment::Environment;
use wave_allocator::Allocator;
use wave_ast::ast::Program;

pub struct Runtime<'a> {
    pub allocator: &'a Allocator,
    pub program: Program<'a>,
    pub env: Environment<'a>,
}

impl<'a> Runtime<'a> {
    pub fn new(allocator: &'a Allocator, env: Environment<'a>, program: Program<'a>) -> Self {
        Self {
            allocator,
            env,
            program,
        }
    }
}
