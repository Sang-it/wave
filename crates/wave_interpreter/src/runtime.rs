use wave_ast::ast::Program;

pub struct Runtime<'a> {
    pub program: Program<'a>,
}

impl<'a> Runtime<'a> {
    pub fn new(program: Program<'a>) -> Self {
        Self { program }
    }
}
