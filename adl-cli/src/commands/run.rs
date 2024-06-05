use codegen::{bytecode::serializer::parse_bytecode_instructions, gen_bytecode};
use colored::*;
use common::util::read_file;
use vm::execute_bytecode;

pub fn run_bytecode(bytecode: String) {
    println!(
        "{}",
        format!("Initiating the virtual machine with bytecode: {}", bytecode)
            .green()
            .bold()
    );
    let source_text = read_file(&bytecode);
    let bytecode = parse_bytecode_instructions(&source_text);
    match bytecode {
        Ok(bytecode) => {
            println!("{}", "Bytecode parsed successfully.".green());
            match execute_bytecode(bytecode) {
                Ok(_) => println!("{}", format!("Compilation result: ()").green()),
                Err(e) => eprintln!("{}", format!("{:?}", e).red()),
            }
        }
        Err(e) => eprintln!("{}", format!("Failed to parse bytecode: {:?}", e).red()),
    }
}

pub fn compile_and_run(input: String) {
    println!(
        "{}",
        format!("Compiling and executing code from: {}", input)
            .green()
            .bold()
    );
    let source_text = read_file(&input);
    match parser::parse(&source_text) {
        Ok(ast) => {
            println!("{}", "Code parsed successfully.".green());
            let bytecode = gen_bytecode(ast);
            println!("{}", "Bytecode generated successfully.".green());
            match execute_bytecode(bytecode) {
                Ok(_) => println!("{}", format!("Compilation result: ()").green()),
                Err(e) => eprintln!("{}", format!("{:?}", e).red()),
            }
        }
        Err(e) => eprintln!("{}", format!("Failed to parse code: {:?}", e).red()),
    }
}
