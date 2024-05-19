use codegen::gen::BytecodeGenerator;
use common::util::read_file;
use parser::ast::visitor::Visitor;
use parser::ast::*;
use vm::VM;

fn main() {
    let source_text =
        read_file("/Users/chernokozinskiy/Documents/address-lang/codegen/test/loop.adl");
    let algo: Algorithm = parser::parse(&source_text).unwrap();

    let mut generator = BytecodeGenerator::new(&algo);
    generator.visit_algorithm(&algo);

    let bytecode = generator.get_bytecode();
    println!("{:?}", bytecode);
    let mut vm = VM::new(bytecode);
    vm.run();
    
}
