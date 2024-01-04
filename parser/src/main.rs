pub mod ast;

pub mod evaluation;
pub mod lexer;
use std::fs;

use lexer::*;
use ast::*;
use builtins::*;
use evaluation::*;

use evaluation::environment::Environment;
use evaluation::value::*;
#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

 
fn main() {
    let test = read_test();
    let mut env = Environment::new();
    env.add_function("Print", Value::Function { function: print_ });
    let lexer = Lexer::new(&test);
    // for item in lexer {
    //     println!("{:?}", item);
    // }
    let ast: Algorithm = grammar::AlgorithmParser::new().parse(lexer).unwrap();
    let lines = match ast {
        Algorithm::Body { lines } => lines,
    };
    // println!("{:?}", ast);
    let mut  compiler = Compiler::new(lines, env);
    let result = compiler.compile();
    println!("{:?}", result);
}

fn read_test() -> String {
    // let f = File::open("examples/test.adl").unwrap();
    let f = "examples/test.adl";
    // let mut lines = BufReader::new(f).lines();
    // let mut test = String::new();

    // loop {
    //     if let Some(s) = lines.next() {
    //         test.push_str(&s.unwrap());
    //     } else {
    //         break;
    //     }
    // }

    let contents = fs::read_to_string(&f)
        .expect("Should have been able to read the file");
    return contents;
}
