use common::util::read_file;
use parser::ast::*;

fn main() {
    let source_text = read_file("examples/subprogram/list.adl");
    let ast: Algorithm = parser::parse(&source_text).unwrap();

    println!("{:?}", ast);
}
