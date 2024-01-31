use crate::evaluation::*;

pub fn print_ (args: Vec<Value>) -> Result<Value, String> {
    for arg in args {
    print!("{}", arg);
    }
    print!("\n");
    Ok(Value:: Null)
}

pub fn to_string_ (args: Vec<Value>) -> Result<Value, String> {
    Ok(Value::new_string(format!("{}", args[0])))
}