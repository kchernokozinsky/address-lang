use crate::VM;
use value::Value;
pub mod string;
pub mod print;

pub type BuiltinFunction = fn(&mut VM, Vec<Value>) -> Value;

