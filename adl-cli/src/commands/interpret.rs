use colored::*;
use common::util::read_file;
use interpreter::interpret;

pub fn run(input: String) {
    println!("{}", format!("Interpreting code from: {}", input).green());
    let source_text = read_file(&input);
    let result = interpret(source_text);
    println!(
        "{}",
        format!("Interpretation result: {:?}", result)
            .green()
            .bold()
    );
}
