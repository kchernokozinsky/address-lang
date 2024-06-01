use value::Value;

use crate::vm::VM;

pub fn builtin_char_at(vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() != 2 {
        panic!("charAt() takes exactly two arguments");
    }
    if let (Value::String(s), Value::Int(index)) = (&args[0], &args[1]) {
        if *index < 0 || *index as usize >= s.len() {
            panic!("Index out of bounds");
        }
        Value::String(s.chars().nth(*index as usize).unwrap().to_string())
    } else {
        panic!("Invalid arguments for charAt()");
    }
}

pub fn builtin_concat(vm: &mut VM, args: Vec<Value>) -> Value {
    let mut result: String = String::new();
    for arg in args {
        result.push_str(&arg.to_string())
    }

    Value::String(result)
}

pub fn builtin_replace(vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() != 3 {
        panic!("replace() takes exactly three arguments");
    }
    if let (Value::String(s), Value::String(old), Value::String(new)) =
        (&args[0], &args[1], &args[2])
    {
        Value::String(s.replace(old, new))
    } else {
        panic!("Invalid arguments for replace()");
    }
}

pub fn builtin_substring(vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() != 3 {
        panic!("substring() takes exactly three arguments");
    }
    if let (Value::String(s), Value::Int(start), Value::Int(end)) = (&args[0], &args[1], &args[2]) {
        if *start < 0 || *end > s.len() as i64 || *start > *end {
            panic!("Invalid range for substring()");
        }
        Value::String(s[*start as usize..*end as usize].to_string())
    } else {
        panic!("Invalid arguments for substring()");
    }
}
