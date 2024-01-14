use std::{fs::File, io::Write, path::Path};

use wave_allocator::Allocator;
use wave_parser::Parser;

fn main() {
    let name = "test.wv".to_string();
    let path = Path::new(&name);
    let source_text = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("{name} not found"));
    let allocator = Allocator::default();
    let ret = Parser::new(&allocator, &source_text).parse();

    let parsed = serde_json::to_string_pretty(&ret.program).unwrap();

    if ret.errors.is_empty() {
        let mut f = File::create("foo.txt").unwrap();
        f.write_all(parsed.as_bytes()).unwrap();
        println!("Parsed Successfully.");
    } else {
        for error in ret.errors {
            let error = error.with_source_code(source_text.clone());
            println!("{error:?}");
        }
    }
}
