use std::collections::HashMap;

use value::Value;

#[derive(Debug)]
pub struct Heap {
    values_by_address: HashMap<i64, Value>,
    next_address: i64,
    free_list: Vec<i64>,
}

impl Heap {
    pub fn new() -> Heap {
        Self {
            values_by_address: HashMap::new(),
            next_address: 0,
            free_list: Vec::new(),
        }
    }

    pub fn allocate_address(&mut self) -> i64 {
        if let Some(address) = self.free_list.pop() {
            address
        } else {
            let address = self.next_address;
            self.next_address += 1;
            if self.is_address_free(address) {
                self.values_by_address.insert(address, Value::Null);
                address
            } else {
                self.allocate_address()
            }
        }
    }

    pub fn lookup_address(&self, address: i64) -> Option<Value> {
        self.values_by_address.get(&address).cloned()
    }

    pub fn allocate_consecutive_addresses(&mut self, count: usize) -> Vec<i64> {
        for start in 0..self.next_address {
            if self.is_block_free(start, count) {
                let addresses = (start..start + count as i64).collect::<Vec<_>>();
                for &address in &addresses {
                    self.free_list.retain(|&x| x != address);
                }
                return addresses;
            }
        }
        let start_address = self.next_address;
        self.next_address += count as i64;
        (start_address..start_address + count as i64).collect()
    }

    fn is_address_free(&self, address: i64) -> bool {
        !self.values_by_address.contains_key(&address)
    }

    fn is_block_free(&self, start: i64, count: usize) -> bool {
        for i in 0..count {
            if self.values_by_address.contains_key(&(start + i as i64))
                || self.free_list.contains(&(start + i as i64))
            {
                return false;
            }
        }
        true
    }
    pub fn store(&mut self, address: i64, value: Value) {
        self.values_by_address.insert(address, value);
    }

    pub fn store_value(&mut self, value: Value) -> i64 {
        let address = self.allocate_address();
        self.values_by_address.insert(address, value);
        address
    }

    pub fn free(&mut self, address: i64) {
        self.values_by_address.remove(&address);
        self.free_list.push(address);
    }

    pub fn lookup_values(&self, values: Vec<Value>) -> Vec<Value> {
        let selected = self
            .values_by_address
            .iter()
            .filter(|(_key, v)| values.contains(v))
            .map(|(&key, _value)| Value::new_int(key.clone()))
            .collect();

        selected
    }
}
