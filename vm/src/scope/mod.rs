use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Scope {
    variable_addresses: HashMap<String, i64>,
}

#[derive(Debug)]
pub enum ScopeError {
    VariableNotFound(String),
}

impl Scope {
    pub fn new() -> Self {
        Self {
            variable_addresses: HashMap::new(),
        }
    }

    pub fn get_var(&self, name: &str) -> Result<i64, ScopeError> {
        self.variable_addresses
            .get(name)
            .cloned()
            .ok_or(ScopeError::VariableNotFound(name.to_string()))
    }

    pub fn set_var(&mut self, name: &str, address: i64) -> Result<(), ScopeError> {
        self.variable_addresses.insert(name.to_string(), address);
        Ok(())
    }
}
