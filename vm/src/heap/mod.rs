use std::collections::HashMap;
use value::Value;

#[derive(Debug)]
pub struct Heap {
    values_by_address: HashMap<i64, Value>,
    next_address_general: i64,
    next_address_reserved: i64,
    free_list_general: Vec<i64>,
    free_list_reserved: Vec<i64>,
    total_limit: i64,
    general_limit: i64,
    reserved_limit: i64,
    general_allocated: i64,
    reserved_allocated: i64,
}

#[derive(Debug)]
pub enum HeapError {
    OutOfMemory,
    InvalidAddress(i64),
    PartitionLimitExceeded,
}

impl Heap {
    pub fn new(total_limit: i64, reserved_ratio: f64) -> Heap {
        let reserved_limit = (total_limit as f64 * reserved_ratio) as i64;
        let general_limit = total_limit - reserved_limit;

        Self {
            values_by_address: HashMap::new(),
            next_address_general: 0,
            next_address_reserved: reserved_limit, // Start reserved addresses from reserved_limit
            free_list_general: Vec::new(),
            free_list_reserved: Vec::new(),
            total_limit,
            general_limit,
            reserved_limit,
            general_allocated: 0,
            reserved_allocated: 0,
        }
    }

    pub fn allocate_address(&mut self, reserved: bool) -> Result<i64, HeapError> {
        if reserved && self.reserved_allocated >= self.reserved_limit {
            return Err(HeapError::PartitionLimitExceeded);
        } else if !reserved && self.general_allocated >= self.general_limit {
            return Err(HeapError::PartitionLimitExceeded);
        }

        let address = if reserved {
            if let Some(address) = self.free_list_reserved.pop() {
                self.reserved_allocated += 1;
                address
            } else {
                let address = self.next_address_reserved;
                self.next_address_reserved += 1;
                self.reserved_allocated += 1;
                address
            }
        } else {
            if let Some(address) = self.free_list_general.pop() {
                self.general_allocated += 1;
                address
            } else {
                let address = self.next_address_general;
                self.next_address_general += 1;
                self.general_allocated += 1;
                address
            }
        };
        if self.is_address_free(address) {
            self.values_by_address.insert(address, Value::Null);
            Ok(address)
        } else {
            self.allocate_address(reserved)
        }
    }

    pub fn lookup_address(&self, address: i64) -> Result<Value, HeapError> {
        self.values_by_address
            .get(&address)
            .cloned()
            .ok_or(HeapError::InvalidAddress(address))
    }

    pub fn allocate_consecutive_addresses(
        &mut self,
        count: usize,
        reserved: bool,
    ) -> Result<Vec<i64>, HeapError> {
        if reserved && (self.reserved_allocated + count as i64) > self.reserved_limit {
            return Err(HeapError::PartitionLimitExceeded);
        } else if !reserved && (self.general_allocated + count as i64) > self.general_limit {
            return Err(HeapError::PartitionLimitExceeded);
        }

        for start in 0..self.next_address_general {
            if self.is_block_free(start, count) {
                let addresses = (start..start + count as i64).collect::<Vec<_>>();
                for &address in &addresses {
                    if reserved {
                        self.free_list_reserved.retain(|&x| x != address);
                    } else {
                        self.free_list_general.retain(|&x| x != address);
                    }
                }
                if reserved {
                    self.reserved_allocated += count as i64;
                } else {
                    self.general_allocated += count as i64;
                }
                return Ok(addresses);
            }
        }

        let start_address = if reserved {
            let start_address = self.next_address_reserved;
            self.next_address_reserved += count as i64;
            self.reserved_allocated += count as i64;
            start_address
        } else {
            let start_address = self.next_address_general;
            self.next_address_general += count as i64;
            self.general_allocated += count as i64;
            start_address
        };

        Ok((start_address..start_address + count as i64).collect())
    }

    fn is_address_free(&self, address: i64) -> bool {
        !self.values_by_address.contains_key(&address)
    }

    fn is_block_free(&self, start: i64, count: usize) -> bool {
        for i in 0..count {
            if self.values_by_address.contains_key(&(start + i as i64))
                || self.free_list_general.contains(&(start + i as i64))
                || self.free_list_reserved.contains(&(start + i as i64))
            {
                return false;
            }
        }
        true
    }

    pub fn store(&mut self, address: i64, value: Value) -> Result<(), HeapError> {
        if address < self.total_limit {
            self.values_by_address.insert(address, value);
            Ok(())
        } else {
            Err(HeapError::InvalidAddress(address))
        }
    }

    pub fn store_value(&mut self, value: Value, reserved: bool) -> Result<i64, HeapError> {
        let address = self.allocate_address(reserved)?;
        self.values_by_address.insert(address, value);
        Ok(address)
    }

    pub fn free(&mut self, address: i64, reserved: bool) -> Result<(), HeapError> {
        if self.values_by_address.contains_key(&address) {
            self.values_by_address.remove(&address);
            if reserved {
                self.free_list_reserved.push(address);
                self.reserved_allocated -= 1;
            } else {
                self.free_list_general.push(address);
                self.general_allocated -= 1;
            }
            Ok(())
        } else {
            Err(HeapError::InvalidAddress(address))
        }
    }

    pub fn lookup_values_general(&self, values: Vec<Value>) -> Vec<Value> {
        let selected = self
            .values_by_address
            .iter()
            .filter(|(&key, v)| values.contains(v) && key < self.reserved_limit)
            .map(|(&key, _value)| Value::new_int(key))
            .collect();

        selected
    }
}
