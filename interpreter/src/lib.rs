use evaluation::{builtins::{print_, to_string_}, errors::EvaluationErrorPrinter, runtime_context::RuntimeContext, Evaluator};
use parser::ast::Algorithm;
use value::Value;

pub mod evaluation;

pub fn interpret(source_text: String) {
    let mut env = RuntimeContext::new();
    env.add_function("Print", Value::new_function(print_));
    env.add_function("Str", Value::new_function(to_string_));

    let ast: Algorithm = parser::parse(&source_text).unwrap();

    let lines = match ast {
        Algorithm::Body(lines) => lines,
    };

    let mut eval = Evaluator::new(lines, env);
    let result = eval.eval();
    match result {
        Ok(_) => {}
        Err(e) => EvaluationErrorPrinter::new(source_text).print_error(&e),
    }
}