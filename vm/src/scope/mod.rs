use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Scope {
    variable_addresses: HashMap<String, i64>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            variable_addresses: HashMap::new(),
        }
    }

    pub fn get_var(&self, name: &str) -> Option<i64> {
        self.variable_addresses.get(name).cloned()
    }

    pub fn set_var(&mut self, name: &str, address: i64) {
        self.variable_addresses.insert(name.to_string(), address);
    }
}
