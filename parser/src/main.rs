pub mod ast;
pub mod evaluation;
pub mod lexer;
pub mod location;
pub mod util;

use lexer::*;
use util::*;
use ast::*;
use builtins::*;
use evaluation::*;

use evaluation::environment::Environment;
use evaluation::value::*;
#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

 
fn main() {
    let test = read_file("examples/list/list_no_syntax_sugar.adl");
    let mut env = Environment::new();
    env.add_function("Print", Value::Function { function: print_ });
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
    println!("{:?}", result);
}

