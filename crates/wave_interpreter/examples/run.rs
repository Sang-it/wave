use std::env;
use wave_allocator::Allocator;
use wave_interpreter::environment::Environment;
use wave_interpreter::eval::Eval;
use wave_interpreter::runtime::Runtime;
use wave_parser::Parser;

fn main() {
    let path = env::current_dir()
        .expect("failed to get current directory")
        .join("wave_interpreter/examples/source.wv");

    let source_text = std::fs::read_to_string(path).expect("failed to read source file");
    let allocator = Allocator::default();
    let ret = Parser::new(&allocator, &source_text).parse();

    if ret.errors.is_empty() {
        let program = ret.program;
        let env = Environment::default();
        let runtime = Runtime::new(env, program);
        let result = runtime.eval();
        match result {
            Ok(result) => println!("{result:?}"),
            Err(error) => println!("{error:?}"),
        }
    } else {
        for error in ret.errors {
            let error = error.with_source_code(source_text.clone());
            println!("{error:?}");
        }
    }
}
