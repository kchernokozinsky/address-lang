use common::util::read_file;
use evaluation::{
    builtins::*, errors::EvaluationErrorPrinter, runtime_context::RuntimeContext, Evaluator,
};
use parser::ast::Algorithm;
use value::*;

pub mod evaluation;

fn main() {
    let mut env = RuntimeContext::new();
    env.add_function("Print", Value::new_function(print_));
    env.add_function("Str", Value::new_function(to_string_));

    let source_text = read_file("examples/loop/simple_loop.adl");
    let ast: Algorithm = parser::parse(&source_text).unwrap();

    // println!("{:?}", ast);

    let lines = match ast {
        Algorithm::Body(lines) => lines,
    };

    let mut compiler = Evaluator::new(lines, env);
    let result = compiler.eval();
    match result {
        Ok(_) => {}
        Err(e) => EvaluationErrorPrinter::new(source_text).print_error(&e),
    }
}
