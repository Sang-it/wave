use std::{cell::RefCell, rc::Rc};

use crate::{
    diagnostics,
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

        fn print(arg: &[Primitive]) -> Result<Primitive<'static>> {
            println!("{:?}", arg);
            Ok(Primitive::Null)
        }

        fn append(arg: &[Primitive]) -> Result<Primitive<'static>> {
            let Primitive::Array(array) = &arg[0] else {
                return Err(diagnostics::NotAnArray().into());
            };

            let mut array = array.clone();

            for primitive in arg.iter().skip(1) {
                array.push(primitive.clone());
            }
            unsafe {
                return Ok(Primitive::Array(std::mem::transmute::<
                    Vec<Primitive>,
                    Vec<Primitive<'static>>,
                >(array)));
            };
        }

        fn contains(arg: &[Primitive]) -> Result<Primitive<'static>> {
            let Primitive::Array(array) = &arg[0] else {
                return Err(diagnostics::NotAnArray().into());
            };
            let value = &arg[1];
            Ok(Primitive::Boolean(array.contains(value)))
        }

        inbuilt_functions.push(InbuiltFunction {
            name: "print".into(),
            function: print,
        });
        inbuilt_functions.push(InbuiltFunction {
            name: "append".into(),
            function: append,
        });
        inbuilt_functions.push(InbuiltFunction {
            name: "contains".into(),
            function: contains,
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
