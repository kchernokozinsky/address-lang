use std::io;

use bytecode::{serializer::write_bytecode_to_file, Bytecode};
use gen::BytecodeGenerator;
use parser::ast::{visitor::Visitor, Algorithm};

pub mod bytecode;
mod gen;

pub fn gen_bytecode(ast: Algorithm) -> Vec<Bytecode> {
    let mut generator = BytecodeGenerator::new(&ast);
    generator.visit_algorithm(&ast);
    generator.get_bytecode()
}

pub fn gen_bytecode_to_file(ast: Algorithm, file_path: &str) -> Result<(), io::Error> {
    let mut generator = BytecodeGenerator::new(&ast);
    generator.visit_algorithm(&ast);
    write_bytecode_to_file(&generator.get_bytecode(), file_path)
}
