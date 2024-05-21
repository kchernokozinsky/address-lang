use codegen::gen::BytecodeGenerator;
use common::util::read_file;
use parser::ast::visitor::Visitor;
use parser::ast::*;
use vm::vm::builtins::builtin_print;
use vm::vm::VM;

fn main() {
    env_logger::init();
    let source_text =
        read_file("/Users/chernokozinskiy/Documents/address-lang/examples/list/concat.adl");
    let algo: Algorithm = parser::parse(&source_text).unwrap();

    let mut generator = BytecodeGenerator::new(&algo);
    generator.visit_algorithm(&algo);

    let bytecode = generator.get_bytecode();
    println!("{:?}", bytecode);
    let mut vm = VM::new(bytecode);
    vm.register_builtin("Print", builtin_print);
    vm.run();
}

// RUST_LOG=trace cargo run