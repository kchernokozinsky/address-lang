use crate::evaluation::*;

pub fn print_ (args: Vec<Value>) -> Result<Value, String> {
    print!("{:?}\n", args);
    Ok(Value:: NIL)
}