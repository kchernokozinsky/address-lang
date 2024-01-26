use crate::evaluation::*;

pub fn print_ (args: Vec<Value>) -> Result<Value, String> {
    for arg in args {
    print!("{}", arg);
    }
    print!("\n");
    Ok(Value:: Null)
}