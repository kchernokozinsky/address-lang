pub mod error;
pub mod typings;
use typings::Type;

use crate::error::ValueError;

use std::fmt::{self};

#[derive(Debug, Clone, PartialOrd)]
pub enum Value {
    Null,
    Float(f64),
    String(String),
    Bool(bool),
    Int(i64),
    Function(fn(Vec<Value>) -> Result<Value, String>),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Null, Value::Null) => true,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Int(a), Value::Int(b)) => a == b,
            // Comparing function pointers directly
            (Value::Function(a), Value::Function(b)) => a as *const _ == b as *const _,
            _ => false,
        }
    }
}

impl Eq for Value {}

impl Value {
    pub fn new_int(value: i64) -> Value {
        Value::Int(value)
    }

    pub fn new_float(value: f64) -> Value {
        Value::Float(value)
    }

    pub fn new_bool(value: bool) -> Value {
        Value::Bool(value)
    }

    pub fn new_string(value: String) -> Value {
        Value::String(value)
    }

    pub fn new_function(function: fn(Vec<Value>) -> Result<Value, String>) -> Value {
        Value::Function(function)
    }

    pub fn type_of(value: &Value) -> Type {
        match value {
            Value::Null => Type::Null,
            Value::Float(_) => Type::Float,
            Value::String(_) => Type::String,
            Value::Bool(_) => Type::Bool,
            Value::Int(_) => Type::Int,
            Value::Function(_) => Type::Function,
        }
    }

    pub fn extract_int(&self) -> Result<i64, ValueError> {
        match self {
            Value::Int(value) => Ok(*value),
            _ => Err(Value::raise_unexpected_type_error(Type::Int, self)),
        }
    }

    pub fn extract_float(&self) -> Result<f64, ValueError> {
        match self {
            Value::Float(value) => Ok(*value),
            _ => Err(Value::raise_unexpected_type_error(Type::Float, self)),
        }
    }

    pub fn extract_bool(&self) -> Result<bool, ValueError> {
        match self {
            Value::Bool(value) => Ok(*value),
            _ => Err(Value::raise_unexpected_type_error(Type::Bool, self)),
        }
    }

    pub fn extract_string(&self) -> Result<String, ValueError> {
        match self {
            Value::String(value) => Ok(value.clone()),
            _ => Err(Value::raise_unexpected_type_error(Type::String, self)),
        }
    }

    pub fn sum(lv: &Value, rv: &Value) -> Result<Value, ValueError> {
        match (lv, rv) {
            (Value::Int(lv), Value::Int(rv)) => Ok(Value::Int(lv + rv)),
            (Value::Float(lv), Value::Float(rv)) => Ok(Value::Float(lv + rv)),
            (Value::String(lv), Value::String(rv)) => Ok(Value::String(lv.to_string() + rv)),
            _ => Err(Value::raise_incompatible_types_error(
                &lv,
                &rv,
                "+".to_owned(),
            )),
        }
    }

    pub fn and(lv: &Value, rv: &Value) -> Result<Value, ValueError> {
        match (lv, rv) {
            (Value::Bool(lv), Value::Bool(rv)) => Ok(Value::Bool(*lv && *rv)),
            _ => Err(Value::raise_incompatible_types_error(
                lv,
                rv,
                "and".to_owned(),
            )),
        }
    }

    pub fn or(lv: &Value, rv: &Value) -> Result<Value, ValueError> {
        match (lv, rv) {
            (Value::Bool(lv), Value::Bool(rv)) => Ok(Value::Bool(*lv || *rv)),
            _ => Err(Value::raise_incompatible_types_error(
                lv,
                rv,
                "and".to_owned(),
            )),
        }
    }

    pub fn mul(lv: &Value, rv: &Value) -> Result<Value, ValueError> {
        match (lv, rv) {
            (Value::Int(lv), Value::Int(rv)) => Ok(Value::Int(lv * rv)),
            (Value::Float(lv), Value::Float(rv)) => Ok(Value::Float(lv * rv)),
            _ => Err(Value::raise_incompatible_types_error(
                &lv,
                &rv,
                "*".to_owned(),
            )),
        }
    }

    pub fn div(lv: &Value, rv: &Value) -> Result<Value, ValueError> {
        match (lv, rv) {
            (Value::Int(lv), Value::Int(rv)) => Ok(Value::Int(lv / rv)),
            (Value::Float(lv), Value::Float(rv)) => Ok(Value::Float(lv / rv)),
            _ => Err(Value::raise_incompatible_types_error(
                &lv,
                &rv,
                "*".to_owned(),
            )),
        }
    }

    pub fn sub(lv: &Value, rv: &Value) -> Result<Value, ValueError> {
        match (lv, rv) {
            (Value::Int(lv), Value::Int(rv)) => Ok(Value::Int(lv.wrapping_sub(*rv))),
            (Value::Float(lv), Value::Float(rv)) => Ok(Value::Float(lv - rv)),
            _ => Err(Value::raise_incompatible_types_error(
                &lv,
                &rv,
                "-".to_owned(),
            )),
        }
    }

    pub fn eq(&self, other: &Value) -> Result<Value, ValueError> {
        match (self, other) {
            (Value::Bool(lhs), Value::Bool(rhs)) => Ok(Value::Bool(lhs == rhs)),
            (Value::Int(lhs), Value::Int(rhs)) => Ok(Value::Bool(lhs == rhs)),
            (Value::Float(lhs), Value::Float(rhs)) => Ok(Value::Bool(lhs == rhs)),
            (Value::String(lhs), Value::String(rhs)) => Ok(Value::Bool(lhs == rhs)),
            (Value::Null, Value::Null) => Ok(Value::Bool(true)),
            _ => Ok(Value::Bool(false)),
        }
    }

    pub fn ne(&self, other: &Value) -> Result<Value, ValueError> {
        match (self, other) {
            (Value::Bool(lhs), Value::Bool(rhs)) => Ok(Value::Bool(lhs != rhs)),
            (Value::Int(lhs), Value::Int(rhs)) => Ok(Value::Bool(lhs != rhs)),
            (Value::Float(lhs), Value::Float(rhs)) => Ok(Value::Bool(lhs != rhs)),
            (Value::String(lhs), Value::String(rhs)) => Ok(Value::Bool(lhs != rhs)),
            (Value::Null, Value::Null) => Ok(Value::Bool(false)),
            _ => Ok(Value::Bool(true)),
        }
    }

    pub fn lt(&self, other: &Value) -> Result<Value, ValueError> {
        match (self, other) {
            (Value::Int(lhs), Value::Int(rhs)) => Ok(Value::Bool(lhs < rhs)),
            (Value::Float(lhs), Value::Float(rhs)) => Ok(Value::Bool(lhs < rhs)),
            (Value::String(lhs), Value::String(rhs)) => Ok(Value::Bool(lhs < rhs)),
            _ => Err(Value::raise_incompatible_types_error(
                self,
                other,
                "<".to_owned(),
            )),
        }
    }

    pub fn le(&self, other: &Value) -> Result<Value, ValueError> {
        match (self, other) {
            (Value::Int(lhs), Value::Int(rhs)) => Ok(Value::Bool(lhs <= rhs)),
            (Value::Float(lhs), Value::Float(rhs)) => Ok(Value::Bool(lhs <= rhs)),
            (Value::String(lhs), Value::String(rhs)) => Ok(Value::Bool(lhs <= rhs)),
            _ => Err(Value::raise_incompatible_types_error(
                self,
                other,
                "<=".to_owned(),
            )),
        }
    }

    pub fn gt(&self, other: &Value) -> Result<Value, ValueError> {
        match (self, other) {
            (Value::Int(lhs), Value::Int(rhs)) => Ok(Value::Bool(lhs > rhs)),
            (Value::Float(lhs), Value::Float(rhs)) => Ok(Value::Bool(lhs > rhs)),
            (Value::String(lhs), Value::String(rhs)) => Ok(Value::Bool(lhs > rhs)),
            _ => Err(Value::raise_incompatible_types_error(
                self,
                other,
                ">".to_owned(),
            )),
        }
    }

    pub fn ge(&self, other: &Value) -> Result<Value, ValueError> {
        match (self, other) {
            (Value::Int(lhs), Value::Int(rhs)) => Ok(Value::Bool(lhs >= rhs)),
            (Value::Float(lhs), Value::Float(rhs)) => Ok(Value::Bool(lhs >= rhs)),
            (Value::String(lhs), Value::String(rhs)) => Ok(Value::Bool(lhs >= rhs)),
            _ => Err(Value::raise_incompatible_types_error(
                self,
                other,
                ">=".to_owned(),
            )),
        }
    }

    pub fn negate(&self) -> Result<Value, ValueError> {
        match self {
            Value::Int(value) => Ok(Value::Int(-value)),
            Value::Float(value) => Ok(Value::Float(-value)),
            _ => Err(Value::raise_unexpected_type_error(Type::Int, self)),
        }
    }

    pub fn modulus(lv: &Value, rv: &Value) -> Result<Value, ValueError> {
        match (lv, rv) {
            (Value::Int(lv), Value::Int(rv)) => Ok(Value::Int(lv % rv)),
            (Value::Float(lv), Value::Float(rv)) => Ok(Value::Float(lv % rv)),
            _ => Err(Value::raise_incompatible_types_error(
                lv,
                rv,
                "%".to_owned(),
            )),
        }
    }

    pub fn not(&self) -> Result<Value, ValueError> {
        match self {
            Value::Bool(value) => Ok(Value::Bool(!value)),
            _ => Err(Value::raise_unexpected_type_error(Type::Bool, self)),
        }
    }

    fn raise_incompatible_types_error(v1: &Value, v2: &Value, op: String) -> ValueError {
        ValueError::IncompatibleTypes {
            operation: op,
            lhs_type: Value::type_of(v1),
            lhs_value: format!("{}", v1),
            rhs_type: Value::type_of(v2),
            rhs_value: format!("{}", v2),
        }
    }

    pub fn raise_unexpected_type_error(expected_types: Type, actual: &Value) -> ValueError {
        ValueError::UnexpectedType {
            expected_type: expected_types,
            actual_type: Value::type_of(actual),
            actual_value: format!("{}", actual),
        }
    }

    pub fn _raise_unexpected_type_error(expected_types: Vec<Type>, actual: &Value) -> ValueError {
        ValueError::_UnexpectedType {
            expected_types: expected_types,
            actual_type: Value::type_of(actual),
            actual_value: format!("{}", actual),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Null => write!(f, "Null"),
            Value::Bool(value) => write!(f, "{}", value),
            Value::Int(value) => write!(f, "{}", value),
            Value::Function(_) => write!(f, "Function"),
            Value::String(value) => write!(f, "{}", value),
            Value::Float(value) => write!(f, "{}", value),
        }
    }
}
