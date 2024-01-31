use std::fmt::{Display, Formatter};
#[derive(Debug, Clone)]
pub enum Type {
    Null,
    Float,
    String,
    Bool,
    Int,
    Function,
    Error,
    Unresolved,
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let type_name = match self {
            Type::Null => "null",
            Type::Float => "float",
            Type::String => "string",
            Type::Int => "int",
            Type::Bool => "bool",
            Type::Function => "function",
            Type::Error => "?",
            Type::Unresolved => "Unresolved",
        };

        write!(f, "{}", type_name)
    }
}

impl Type {

    pub fn from_str(s: &str) -> Option<Type> {
        match s {
            "null" => Some(Type::Null),
            "float" => Some(Type::Float),
            "string" => Some(Type::String),
            "bool" => Some(Type::Bool),
            "int" => Some(Type::Int),
            _ => None,
        }
    }
}