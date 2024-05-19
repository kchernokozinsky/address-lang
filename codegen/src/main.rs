use codegen::bytecode::io::write_bytecode_to_file;
use codegen::gen::BytecodeGenerator;
use common::util::read_file;
use parser::ast::visitor::Visitor;
use parser::ast::Algorithm;

fn main() {
    let file_path = "codegen/test/bytecode/loop_bytecode.txt";
    let source_text = read_file("codegen/test/loop.adl");
    let algo: Algorithm = parser::parse(&source_text).unwrap();

    let mut generator = BytecodeGenerator::new(&algo);
    generator.visit_algorithm(&algo);

    let bytecode = generator.get_bytecode();
    println!("{:?}", bytecode);

    write_bytecode_to_file(&bytecode, file_path).expect("Failed to write bytecode to file");
}
