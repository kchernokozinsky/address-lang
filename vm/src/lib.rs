pub mod builtins;
pub mod scope;
pub mod vm;

use builtins::builtin_print;
use codegen::bytecode::Bytecode;
use vm::VM;

pub fn execute_bytecode(bytecode: Vec<Bytecode>) {
    let mut vm = VM::new(bytecode);
    vm.register_builtin("Print", builtin_print);
    vm.run();
}
