use codegen::bytecode::Bytecode;
use value::Value;
use vm::execute_bytecode;

fn main() {
    env_logger::init();
    let bytecode = vec![
        Bytecode::Constant(Value::new_int(5)),
        Bytecode::BindAddr("x".to_string()),
        Bytecode::LoadVar("x".to_string()),
        Bytecode::Constant(Value::new_int(3)),
        Bytecode::Add,
        Bytecode::BindAddr("y".to_string()),
        Bytecode::LoadVar("y".to_string()),
        Bytecode::Halt,
    ];
    execute_bytecode(bytecode);
}
// RUST_LOG=trace cargo run
