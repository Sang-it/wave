use std::{cell::RefCell, rc::Rc};

use crate::{
    environment::Environment,
    evaluator::{function::InbuiltFunction, Primitive},
};
use wave_ast::ast::Program;
use wave_diagnostics::Result;

pub struct Runtime<'a> {
    pub program: Program<'a>,
    pub inbuilt_functions: Vec<InbuiltFunction>,
}

impl<'a> Runtime<'a> {
    pub fn new(program: Program<'a>) -> Self {
        let mut inbuilt_functions = vec![];

        fn print(arg: &Vec<Primitive>) -> Primitive<'static> {
            println!("{:?}", arg);
            Primitive::Null
        }

        inbuilt_functions.push(InbuiltFunction {
            name: "print".into(),
            function: print,
        });

        Self {
            program,
            inbuilt_functions,
        }
    }

    pub fn eval(&self) -> Result<Primitive<'a>> {
        let environment = Rc::new(RefCell::new(Environment::default()));
        self.eval_program(&self.program, environment)
    }

    pub fn eval_program(
        &self,
        program: &Program<'a>,
        environment: Rc<RefCell<Environment<'a>>>,
    ) -> Result<Primitive<'a>> {
        let mut result = Primitive::Null;
        for statement in &program.body {
            result = self.eval_statement(statement, Rc::clone(&environment))?;
        }
        Ok(result)
    }
}
