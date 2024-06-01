use crate::VM;
use value::Value;
pub mod print;
pub mod string;

pub type BuiltinFunction = fn(&mut VM, Vec<Value>) -> Value;
