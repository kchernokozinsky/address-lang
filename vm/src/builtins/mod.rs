use crate::VM;
use value::Value;

pub type BuiltinFunction = fn(&mut VM, Vec<Value>) -> Value;

pub fn builtin_print(vm: &mut VM, args: Vec<Value>) -> Value {
    for arg in args {
        match arg {
            Value::Int(i) => print!("{}", i),
            Value::Float(f) => print!("{}", f),
            Value::Bool(b) => print!("{}", b),
            Value::String(s) => print!("{}", s),
            Value::Null => print!("null"),
            Value::Function(_) => todo!(),
        }
    }
    println!(); // New line after printing all arguments
    Value::Null // Return null value
}
