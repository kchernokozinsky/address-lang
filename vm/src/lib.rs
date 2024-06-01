pub mod builtins;
pub mod scope;
pub mod vm;

use builtins::{print::builtin_print, string::*};
use codegen::bytecode::Bytecode;
use vm::VM;

pub fn execute_bytecode(bytecode: Vec<Bytecode>) {
    let mut vm = VM::new(bytecode);
    vm.register_builtin("Print", builtin_print);
    vm.register_builtin("CharAt", builtin_char_at);
    vm.register_builtin("Concat", builtin_concat);
    vm.register_builtin("Replace", builtin_replace);
    vm.register_builtin("SubString", builtin_substring);

    vm.run();
}
