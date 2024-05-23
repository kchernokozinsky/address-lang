use colored::*;
use parser::{ast::serializer::serialize_ast_to_file, parse_by_path};

pub fn run(input: String, output: Option<String>) {
    println!("{}", format!("Executing syntactic analysis on: {}", input).green().bold());
    match parse_by_path(&input) {
        Ok(ast) => {
            println!("{}", "Syntactic analysis completed successfully.".green().bold());
            if let Some(output) = output {
                serialize_ast_to_file(&ast, &output).expect("Serialization failed");
                println!("{}", format!("Analysis results have been saved to: {}", output).green().bold());
            } else {
                println!("{}", format!("{:?}", ast).green());
            }
        }
        Err(e) => eprintln!("{}", format!("Syntactic analysis failed: {:?}", e).red().bold()),
    }
}
