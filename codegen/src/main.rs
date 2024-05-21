use codegen::gen_bytecode_to_file;

use common::util::read_file;

use parser::ast::Algorithm;

fn main() {
    let file_path = "codegen/test/bytecode/loop_bytecode.txt";
    let source_text = read_file("codegen/test/adl/loop.adl");
    let algo: Algorithm = parser::parse(&source_text).unwrap();

    let _ = gen_bytecode_to_file(algo, file_path);
}
