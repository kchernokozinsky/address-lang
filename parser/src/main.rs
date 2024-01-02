pub mod ast;
pub mod builtins;
pub mod evaluation;
pub mod lexer;

use lexer::*;
use ast::*;
use builtins::*;
use evaluation::*;
use std::{fs::File, io::BufRead, io::BufReader};
#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);


fn main() {
    let test = read_test();
    let mut env = Environment::new();
    env.add_function("print", Value::Function { function: print_ });
    let lexer = Lexer::new(&test);
    let ast: Algorithm = grammar::AlgorithmParser::new().parse(lexer).unwrap();
    // println!("{:?}", ast);
    let result = eval_algorithm(&mut env, ast);
    println!("{:?}", result);
}

fn read_test() -> String {
    let f = File::open("examples/test.adl").unwrap();
    let mut lines = BufReader::new(f).lines();
    let mut test = String::new();

    loop {
        if let Some(s) = lines.next() {
            test.push_str(&s.unwrap());
        } else {
            break;
        }
    }
    return test;
}
