use crate::environment::Environment;
use wave_ast::ast::Program;

pub struct Runtime<'a> {
    pub program: Program<'a>,
    pub env: Environment<'a>,
}

impl<'a> Runtime<'a> {
    pub fn new(env: Environment<'a>, program: Program<'a>) -> Self {
        Self { env, program }
    }
}
