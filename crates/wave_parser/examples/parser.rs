use wave_allocator::Allocator;
use wave_parser::Parser;

fn main() {
    let source_text = "const x = 1;";
    let allocator = Allocator::default();
    let parser = Parser::new(&allocator, source_text);
    let parser_return = parser.parse();
    dbg!("{:#?}", parser_return.program);
    dbg!("{:?}", parser_return.errors);
}
