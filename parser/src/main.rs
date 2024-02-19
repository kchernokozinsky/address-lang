pub mod ast;
pub mod evaluation;
pub mod lexer;
pub mod location;
pub mod util;
pub mod typings;

use evaluation::errors::{EvaluationErrorPrinter};
use lexer::*;
use util::*;
use ast::*;
use builtins::*;
use evaluation::*;

use evaluation::runtime_context::RuntimeContext;
use evaluation::value::*;
#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

 
fn main() {
    let test = read_file("examples/subprogram/list.adl");
    let mut env = RuntimeContext::new();
    env.add_function("Print", Value::new_function(print_));
    env.add_function("Str", Value::new_function(to_string_));
    let lexer = Lexer::new(&test);

    // for item in lexer {
    //     println!("{:?}", item);
    // }

    let ast: Algorithm = grammar::AlgorithmParser::new().parse(lexer).unwrap();

    // println!("{:?}", ast);

    let lines = match ast {
        Algorithm::Body(lines) => lines,
    };

    let mut  compiler = Evaluator::new(lines, env);
    let result = compiler.eval();
    match result {
        Ok(_) => {},
        Err(e) => EvaluationErrorPrinter::new(test).print_error(&e),
    }
}

