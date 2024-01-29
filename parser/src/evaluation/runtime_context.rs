use std::collections::HashMap;
use crate::value::*;
pub struct RuntimeContext {
    functions: HashMap<String, Value>,
    variable_addresses: HashMap<String, i64>,
    values_by_address: HashMap<i64, Value>,
    labels: HashMap<String, usize>,
}

impl RuntimeContext {
    pub fn new() -> RuntimeContext {
        RuntimeContext {
            functions: HashMap::new(),
            variable_addresses: HashMap::new(),
            values_by_address: HashMap::new(),
            labels: HashMap::new()
        }
    }

    pub fn get_function(&self, name: &str) -> Result<&Value, String> {
        match self.functions.get(name) {
            Some(v) => Ok(v),
            None => return Err(format!("Function '{}' is not defined", name)),
        }
    }

    pub fn add_function(&mut self, name: &str, function: Value) -> () {
        self.functions.insert(name.to_string(), function);
    }

    pub fn add_variable(&mut self, name: &str, address: i64) -> () {
        self.variable_addresses.insert(name.to_string(), address);
    }
    pub fn get_variable_address(&self, name: &str) -> Result<i64, String> {
        self.variable_addresses.get(name)
        .copied()
        .ok_or_else(|| format!("Variable '{}' is not defined", name))
}
    

    pub fn allocate_variable(&mut self, name: &str) -> i64 {
        let address = self.generate_free_address();
        self.variable_addresses.insert(name.to_string(), address);
        address
    }

    pub fn write_to_address(&mut self, address: i64, value: Value) -> () {
        self.values_by_address.insert(address, value);
    }

    pub fn read_from_address(&self, address: i64) -> &Value {
        match self.values_by_address.get(&address) {
            Some(v) => v,
            None => &Value::Null,
        }
    }

    pub fn register_label(&mut self, label: String, line: usize) {
        self.labels.insert(label, line);
    }

    pub fn lookup_line_by_label(&self, label: &String) -> Option<&usize> {
        self.labels.get(label)
    }
    // SHOULD BE REWORKED
    fn generate_free_address(&self) -> i64 {
        let mut address = 0;
        while self.values_by_address.contains_key(&address) {
            address += 1;
        }
        address
    }
}