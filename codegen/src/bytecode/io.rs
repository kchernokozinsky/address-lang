use crate::bytecode::Bytecode;
use std::fs::File;
use std::io::{self, Read, Write};
use value::Value;

/// Serializes bytecode to a human-readable format and writes to a file.
pub fn write_bytecode_to_file(bytecode: &[Bytecode], file_path: &str) -> Result<(), io::Error> {
    let mut file = File::create(file_path)?;
    for (i, instruction) in bytecode.iter().enumerate() {
        let serialized_instruction = format_bytecode_instruction(i, instruction);
        file.write_all(serialized_instruction.as_bytes())?;
    }
    Ok(())
}

/// Deserializes bytecode from a human-readable format in a file.
pub fn read_bytecode_from_file(file_path: &str) -> Result<Vec<Bytecode>, io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let bytecode = parse_bytecode_instructions(&contents)?;
    Ok(bytecode)
}

fn format_bytecode_instruction(offset: usize, instruction: &Bytecode) -> String {
    match instruction {
        Bytecode::Constant(val) => match val {
            Value::Int(i) => format!("{:<5} {:<23} {}\n", offset, "LOAD_CONST", i),
            Value::Float(f) => format!("{:<5} {:<23} {}\n", offset, "LOAD_CONST", f),
            Value::Bool(b) => format!("{:<5} {:<23} {}\n", offset, "LOAD_CONST", b),
            Value::String(s) => format!("{:<5} {:<23} '{}'\n", offset, "LOAD_CONST", s),
            _ => format!("{:<5} {:<23} {:?}\n", offset, "LOAD_CONST", val),
        },
        Bytecode::LoadVar(name) => format!("{:<5} {:<23} {}\n", offset, "LOAD_NAME", name),
        Bytecode::StoreVar(name) => format!("{:<5} {:<23} {}\n", offset, "STORE_NAME", name),
        Bytecode::Add => format!("{:<5} {}\n", offset, "BINARY_ADD"),
        Bytecode::Sub => format!("{:<5} {}\n", offset, "BINARY_SUBTRACT"),
        Bytecode::Mul => format!("{:<5} {}\n", offset, "BINARY_MULTIPLY"),
        Bytecode::Div => format!("{:<5} {}\n", offset, "BINARY_DIVIDE"),
        Bytecode::Mod => format!("{:<5} {}\n", offset, "BINARY_MODULO"),
        Bytecode::And => format!("{:<5} {}\n", offset, "BINARY_AND"),
        Bytecode::Or => format!("{:<5} {}\n", offset, "BINARY_OR"),
        Bytecode::Equal => format!("{:<5} {}\n", offset, "COMPARE_OP EQ"),
        Bytecode::NotEqual => format!("{:<5} {}\n", offset, "COMPARE_OP NE"),
        Bytecode::Greater => format!("{:<5} {}\n", offset, "COMPARE_OP GT"),
        Bytecode::Less => format!("{:<5} {}\n", offset, "COMPARE_OP LT"),
        Bytecode::CallBuiltin(name, arity) => format!(
            "{:<5} {:<23} {} ({})\n",
            offset, "CALL_FUNCTION", name, arity
        ),
        Bytecode::Return => format!("{:<5} {}\n", offset, "RETURN_VALUE"),
        Bytecode::Jump(addr) => format!("{:<5} {:<23} {}\n", offset, "JUMP", addr),
        Bytecode::JumpIfFalse(addr) => format!("{:<5} {:<23} {}\n", offset, "JUMP_IF_FALSE", addr),
        Bytecode::Label(label) => format!("{:<5} {:<23} {}\n", offset, "LABEL", label),
        Bytecode::Halt => format!("{:<5} {}\n", offset, "HALT"),
        Bytecode::Not => format!("{:<5} {}\n", offset, "UNARY_NOT"),
        Bytecode::Negate => format!("{:<5} {}\n", offset, "UNARY_NEGATIVE"),
        Bytecode::Pop => format!("{:<5} {}\n", offset, "POP_TOP"),
        Bytecode::Deref => format!("{:<5} {}\n", offset, "DEREFERENCE"),
        Bytecode::MulDeref => format!("{:<5} {}\n", offset, "MULTIPLE_DEREFERENCE"),
        Bytecode::Store => format!("{:<5} {}\n", offset, "STORE"),
        Bytecode::Alloc => format!("{:<5} {}\n", offset, "ALLOC"),
        Bytecode::Dup => format!("{:<5} {}\n", offset, "DUP"),
        Bytecode::StoreAddr => format!("{:<5} {}\n", offset, "STORE_ADDR"),
        Bytecode::BindAddr(name) => format!("{:<5} {:<23} {}\n", offset, "BIND_ADDR", name),
        Bytecode::FreeAddr => format!("{:<5} {}\n", offset, "FREE_ADDR"),
        Bytecode::CallSubProgram(label, arity) => format!(
            "{:<5} {:<23} {} ({})\n",
            offset,
            "CALL_SUBPROGRAM",
            label,
            arity
        ),
        Bytecode::PushScope => format!("{:<5} {}\n", offset, "PUSH_SCOPE"),
        Bytecode::PopScope => format!("{:<5} {}\n", offset, "POP_SCOPE"),
    }
}

fn parse_bytecode_instructions(contents: &str) -> Result<Vec<Bytecode>, io::Error> {
    let mut bytecode = Vec::new();
    for line in contents.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        let instruction = match parts[1] {
            "LOAD_CONST" => {
                let value = parts[2];
                if let Ok(i) = value.parse::<i64>() {
                    Bytecode::Constant(Value::new_int(i))
                } else if let Ok(f) = value.parse::<f64>() {
                    Bytecode::Constant(Value::new_float(f))
                } else if let Ok(b) = value.parse::<bool>() {
                    Bytecode::Constant(Value::new_bool(b))
                } else {
                    let s = value.trim_matches('\'').to_string();
                    Bytecode::Constant(Value::new_string(s))
                }
            }
            "LOAD_NAME" => Bytecode::LoadVar(parts[2].to_string()),
            "STORE_NAME" => Bytecode::StoreVar(parts[2].to_string()),
            "BIND_ADDR" => Bytecode::BindAddr(parts[2].to_string()),
            "BINARY_ADD" => Bytecode::Add,
            "BINARY_SUBTRACT" => Bytecode::Sub,
            "BINARY_MULTIPLY" => Bytecode::Mul,
            "BINARY_DIVIDE" => Bytecode::Div,
            "BINARY_MODULO" => Bytecode::Mod,
            "BINARY_AND" => Bytecode::And,
            "BINARY_OR" => Bytecode::Or,
            "COMPARE_OP" => match parts[2] {
                "EQ" => Bytecode::Equal,
                "NE" => Bytecode::NotEqual,
                "GT" => Bytecode::Greater,
                "LT" => Bytecode::Less,
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid compare operation",
                    ))
                }
            },
            "CALL_FUNCTION" => {
                if parts.len() < 4 {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid CALL_FUNCTION format",
                    ));
                }
                let name = parts[2].to_string();
                let arity = parts[3]
                    .trim_matches('(')
                    .trim_matches(')')
                    .parse()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                Bytecode::CallBuiltin(name, arity)
            }
            "CALL_SUBPROGRAM" => {
                if parts.len() < 4 {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid CALL_SUBPROGRAM format",
                    ));
                }
                let label = parts[2]
                    .parse()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                let arity = parts[3]
                    .trim_matches('(')
                    .trim_matches(')')
                    .parse()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                Bytecode::CallSubProgram(label, arity)
            }
            "RETURN_VALUE" => Bytecode::Return,
            "JUMP" => {
                let addr = parts[2]
                    .parse()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                Bytecode::Jump(addr)
            }
            "JUMP_IF_FALSE" => {
                let addr = parts[2]
                    .parse()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                Bytecode::JumpIfFalse(addr)
            }
            "LABEL" => Bytecode::Label(parts[2].to_string()),
            "HALT" => Bytecode::Halt,
            "UNARY_NOT" => Bytecode::Not,
            "UNARY_NEGATIVE" => Bytecode::Negate,
            "POP_TOP" => Bytecode::Pop,
            "DEREFERENCE" => Bytecode::Deref,
            "MULTIPLE_DEREFERENCE" => Bytecode::MulDeref,
            "STORE" => Bytecode::Store,
            "ALLOC" => Bytecode::Alloc,
            "DUP" => Bytecode::Dup,
            "FREE_ADDR" => Bytecode::FreeAddr,
            "STORE_ADDR" => Bytecode::StoreAddr,
            "PUSH_SCOPE" => Bytecode::PushScope,
            "POP_SCOPE" => Bytecode::PopScope,

            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Unknown bytecode instruction",
                ))
            }
        };
        bytecode.push(instruction);
    }
    Ok(bytecode)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bytecode::Bytecode;
    use value::Value;

    #[test]
    fn test_write_and_read_bytecode() {
        let bytecode = vec![
            Bytecode::Constant(Value::new_int(42)),
            Bytecode::StoreVar("x".to_string()),
            Bytecode::LoadVar("x".to_string()),
            Bytecode::Add,
            Bytecode::Return,
            Bytecode::Halt,
            Bytecode::Not,
            Bytecode::Negate,
            Bytecode::Pop,
            Bytecode::Deref,
            Bytecode::MulDeref,
            Bytecode::Store,
            Bytecode::CallBuiltin("procedure_name".to_string(), 4),
            Bytecode::CallSubProgram(0, 2),
            Bytecode::Label("start".to_string()),
            Bytecode::Jump(10),
            Bytecode::JumpIfFalse(15),
            Bytecode::Alloc,
            Bytecode::Dup,
            Bytecode::StoreAddr,
            Bytecode::BindAddr("a".to_string()),
            Bytecode::FreeAddr,
            Bytecode::PushScope,
            Bytecode::PopScope,
        ];

        let file_path = "test/bytecode/test_bytecode.txt";

        write_bytecode_to_file(&bytecode, file_path).expect("Failed to write bytecode to file");

        let read_bytecode =
            read_bytecode_from_file(file_path).expect("Failed to read bytecode from file");

        assert_eq!(bytecode, read_bytecode);

        std::fs::remove_file(file_path).expect("Failed to delete test file");
    }
}
