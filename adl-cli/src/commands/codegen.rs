use codegen::{bytecode::serializer::write_bytecode_to_file, gen_bytecode};
use colored::*;
use common::util::read_file;

pub fn run(input: String, output: Option<String>) {
    println!("{}", format!("Generating bytecode from: {}", input).green().bold());
    let source_text = read_file(&input);
    match parser::parse(&source_text) {
        Ok(ast) => {
            println!("{}", "Code parsed successfully.".green().bold());
            let bytecode = gen_bytecode(ast);
            println!("{}", "Bytecode generated successfully.".green().bold());
            if let Some(output) = output {
                write_bytecode_to_file(&bytecode, &output).expect("Serialization failed");
                println!("{}", format!("Bytecode has been saved to: {}", output).green().bold());
            } else {
                println!("{}", format!("{:?}", bytecode).green());
            }
        }
        Err(e) => eprintln!("{}", format!("Failed to parse code: {:?}", e).red().bold()),
    }
}
