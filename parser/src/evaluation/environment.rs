
use std::collections::HashMap;
use crate::value::*;
pub struct Environment {
    function_space: HashMap<String, Value>,
    variable_to_address: HashMap<String, i64>,
    address_to_value: HashMap<i64, Value>,
    labels: HashMap<String, usize>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            function_space: HashMap::new(),
            variable_to_address: HashMap::new(),
            address_to_value: HashMap::new(),
            labels: HashMap::new()
        }
    }

    pub fn get_function(&self, name: &str) -> Result<&Value, String> {
        match self.function_space.get(name) {
            Some(v) => Ok(v),
            None => return Err(format!("Function '{}' is not defined", name)),
        }
    }

    pub fn add_function(&mut self, name: &str, function: Value) -> () {
        self.function_space.insert(name.to_string(), function);
    }

    pub fn add_variable(&mut self, name: &str, address: i64) -> () {
        self.variable_to_address.insert(name.to_string(), address);
    }
    pub fn get_variable(&self, name: &str) -> Result<Value, String> {
        match self.variable_to_address.get(name) {
            Some(v) => Ok(Value::Int {
                value: v.to_owned(),
            }),
            None => return Err(format!("Variable '{}' is not defined", name)),
        }
    }

    pub fn fill_address(&mut self, address: i64, value: Value) -> () {
        self.address_to_value.insert(address, value);
    }

    pub fn get_value_by_address(&self, address: i64) -> Result<&Value, String> {
        match self.address_to_value.get(&address) {
            Some(v) => Ok(v),
            None => return Err(format!("Address '{}' is empty", address)),
        }
    }

    pub fn add_label(&mut self, label: String, line: usize) {
        self.labels.insert(label, line);
    }

    pub fn get_line_by_label(&self, label: &String) -> Option<&usize> {
        self.labels.get(label)
    }
}