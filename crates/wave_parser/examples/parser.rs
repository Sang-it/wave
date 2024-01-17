use std::{env, fs::File, io::Write};

use wave_allocator::Allocator;
use wave_parser::Parser;

fn main() {
    let path = env::current_dir()
        .expect("failed to get current directory")
        .join("wave_parser/examples/source.wv");

    let source_text = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("file not found"));
    let allocator = Allocator::default();
    let ret = Parser::new(&allocator, &source_text).parse();

    let parsed = serde_json::to_string_pretty(&ret.program).unwrap();

    if ret.errors.is_empty() {
        let mut f = File::create("wave_parser/examples/source.txt").unwrap();
        f.write_all(parsed.as_bytes()).unwrap();
        println!("Parsed Successfully.");
    } else {
        for error in ret.errors {
            let error = error.with_source_code(source_text.clone());
            println!("{error:?}");
        }
    }
}
