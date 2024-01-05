use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Null,
    String {
        value: String,
    },
    Bool {
        value: bool,
    },
    Int {
        value: i64,
    },
    Function {
        function: fn(Vec<Value>) -> Result<Value, String>,
    },
}
impl Value {
    pub fn sum(lv: Value, rv: Value) -> Result<Value, String> {
        let lv_ = match lv {
            Value::Int { value } => value,
            _ => return Err(format!("{:?} and {:?} are not compatible", lv, rv))
        };

        let rv_ = match rv {
            Value::Int { value } => value,
            _ => return Err(format!("{:?} and {:?} are not compatible", lv, rv))
        };

        Ok(Value::Int{value: lv_ + rv_})
    }

    pub fn sub(lv: Value, rv: Value) -> Result<Value, String> {
        let lv_ = match lv {
            Value::Int { value } => value,
            _ => return Err(format!("{:?} and {:?} are not compatible", lv, rv))
        };

        let rv_ = match rv {
            Value::Int { value } => value,
            _ => return Err(format!("{:?} and {:?} are not compatible", lv, rv))
        };

        Ok(Value::Int{value: lv_ - rv_})
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Null => write!(f, "Null"),
            Value::Bool { value } => write!(f, "Boolean: {}", value),
            Value::Int { value } => write!(f, "Integer: {}", value),
            Value::Function { .. } => write!(f, "Function"),
            Value::String { value } => write!(f, "String: {}", value),
        }
    }
}