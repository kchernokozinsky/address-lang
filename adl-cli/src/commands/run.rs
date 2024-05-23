use colored::*;
use codegen::{bytecode::serializer::parse_bytecode_instructions, gen_bytecode};
use common::util::read_file;
use vm::execute_bytecode;

pub fn run_bytecode(bytecode: String) {
    println!("{}", format!("Initiating the virtual machine with bytecode: {}", bytecode).green().bold());
    let source_text = read_file(&bytecode);
    let bytecode = parse_bytecode_instructions(&source_text);
    match bytecode {
        Ok(bytecode) => {
            println!("{}", "Bytecode parsed successfully.".green().bold());
            execute_bytecode(bytecode)
        },
        Err(e) => eprintln!("{}", format!("Failed to parse bytecode: {:?}", e).red().bold()),
    }
}

pub fn compile_and_run(input: String) {
    println!("{}", format!("Compiling and executing code from: {}", input).green().bold());
    let source_text = read_file(&input);
    match parser::parse(&source_text) {
        Ok(ast) => {
            println!("{}", "Code parsed successfully.".green().bold());
            let bytecode = gen_bytecode(ast);
            println!("{}", "Bytecode generated successfully.".green().bold());
            execute_bytecode(bytecode);
            println!("{}", format!("Compilation result: ()").green().bold());
        },
        Err(e) => eprintln!("{}", format!("Failed to parse code: {:?}", e).red().bold()),
    }
}
