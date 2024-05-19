use common::util::read_file;
use parser::ast::{serializer::serialize_ast_to_file, *};

fn main() {
    let source_text = read_file("examples/loop/loop.adl");
    let ast: Algorithm = parser::parse(&source_text).unwrap();
    println!("{:?}", serialize_ast_to_file(&ast, "loop.json"));

    println!("{:?}", ast);
}
