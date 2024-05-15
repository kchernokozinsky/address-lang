use value::*;

pub fn print_(args: Vec<Value>) -> Result<Value, String> {
    for arg in args {
        print!("{}", arg);
    }
    println!();
    Ok(Value::Null)
}

pub fn to_string_(args: Vec<Value>) -> Result<Value, String> {
    Ok(Value::new_string(format!("{}", args[0])))
}
