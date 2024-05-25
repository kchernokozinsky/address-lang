use value::Value;

use crate::vm::VM;

pub fn builtin_print(vm: &mut VM, args: Vec<Value>) -> Value {
    for arg in args {
        print!("{}", arg.to_string())
    }
    println!(); // New line after printing all arguments
    Value::Null // Return null value
}
