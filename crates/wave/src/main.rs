use std::env;
use wave_allocator::Allocator;
use wave_interpreter::Runtime;
use wave_parser::Parser;

fn main() -> Result<(), String>{
    let source = check_args()?;

    let path = env::current_dir()
        .expect("failed to get current directory")
        .join(source);

    let source_text = std::fs::read_to_string(path).expect("failed to read source file");
    let allocator = Allocator::default();
    let ret = Parser::new(&allocator, &source_text).parse();

    if ret.errors.is_empty() {
        let program = ret.program;
        let runtime = Runtime::new(program);
        let result = Runtime::eval(&runtime);
        match result {
            Ok(_) => {}
            Err(error) => {
                let error = error.with_source_code(source_text.clone());
                println!("{error:?}");
            }
        }
    } else {
        for error in ret.errors {
            let error = error.with_source_code(source_text.clone());
            println!("{error:?}");
        }
    }
    Ok(())
}

fn check_args() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Usage: wave <filename>".to_string());
    }
    Ok(args[1].clone())
}
