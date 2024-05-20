use std::collections::HashMap;

use codegen::bytecode::Bytecode;
use value::Value;

pub mod builtins;
mod tests;

pub struct VM {
    bytecode: Vec<Bytecode>,
    pc: usize,
    stack: Vec<Value>,
    variable_addresses: HashMap<String, i64>,
    values_by_address: HashMap<i64, Value>,
    builtins: HashMap<String, BuiltinFunction>,
    next_address: i64,   // To keep track of the next free address
    free_list: Vec<i64>, // List of freed addresses
}

type BuiltinFunction = fn(&mut VM, Vec<Value>) -> Value;

impl VM {
    pub fn new(bytecode: Vec<Bytecode>) -> Self {
        Self {
            bytecode,
            pc: 0,
            stack: Vec::new(),
            variable_addresses: HashMap::new(),
            values_by_address: HashMap::new(),
            builtins: HashMap::new(),
            next_address: 0,       // Start with address 0
            free_list: Vec::new(), // Initialize the free list
        }
    }

    pub fn register_builtin(&mut self, name: &str, func: BuiltinFunction) {
        self.builtins.insert(name.to_string(), func);
    }

    pub fn run(&mut self) {
        while self.pc < self.bytecode.len() {
            let instruction = self.bytecode[self.pc].clone();
            self.pc += 1;
            println!("---{:?}---", self.pc);
            println!("{:?}", instruction);

            match instruction {
                Bytecode::Constant(value) => self.stack.push(value),
                Bytecode::LoadVar(name) => self.get_var(&name),
                Bytecode::StoreVar(name) => self.set_var(&name),
                Bytecode::Add => self.binary_op(|a, b| a + b),
                Bytecode::Sub => self.binary_op(|a, b| a - b),
                Bytecode::Mul => self.binary_op(|a, b| a * b),
                Bytecode::Div => self.binary_op(|a, b| a / b),
                Bytecode::Mod => self.binary_op(|a, b| a % b),
                Bytecode::And => self.binary_op_bool(|a, b| a && b),
                Bytecode::Or => self.binary_op_bool(|a, b| a || b),
                Bytecode::Equal => self.binary_op_cmp(|a, b| a == b),
                Bytecode::NotEqual => self.binary_op_cmp(|a, b| a != b),
                Bytecode::Greater => self.binary_op_cmp(|a, b| a > b),
                Bytecode::Less => self.binary_op_cmp(|a, b| a < b),
                Bytecode::Not => self.unary_op_bool(|a| !a),
                Bytecode::Negate => self.unary_op(|a| -a),
                Bytecode::Jump(addr) => self.pc = addr,
                Bytecode::JumpIfFalse(addr) => self.jump_if_false(addr),
                Bytecode::Label(_) => {}
                Bytecode::Call(name, argc) => self.call_builtin(&name, argc),
                Bytecode::Return => return,
                Bytecode::Halt => break,
                Bytecode::Pop => {
                    self.stack.pop();
                }
                Bytecode::Deref => self.deref(),
                Bytecode::MulDeref => self.mul_deref(),
                Bytecode::Store => self.store(),
                Bytecode::CallProc(_) => unimplemented!(),
                Bytecode::CallFn(_) => unimplemented!(),
                Bytecode::Alloc => self.alloc(),
                Bytecode::Dup => self.dup(),
                Bytecode::StoreAddr => self.store_addr(),
                Bytecode::BindAddr(name) => self.bind_addr(name),
                Bytecode::FreeAddr => self.free_addr(),
            }
            println!("stack: {:?}", self.stack);
        }
    }

    fn store_addr(&mut self) {
        let value = self.stack.pop().unwrap();
        let address = self.allocate_address();
        self.values_by_address.insert(address, value);
        self.stack.push(Value::new_int(address));
    }

    fn bind_addr(&mut self, name: String) {
        let address = self.stack.pop().unwrap();
        if let Value::Int(addr) = address {
            self.variable_addresses.insert(name, addr);
        } else {
            panic!("BindAddr operation requires an integer address on the stack");
        }
    }

    fn get_var(&mut self, name: &str) {
        let address = self
            .variable_addresses
            .get(name)
            .cloned()
            .unwrap_or_else(|| {
                let address = self.allocate_address();
                self.variable_addresses.insert(name.to_string(), address);
                address
            });
        self.stack.push(Value::Int(address));
    }

    fn set_var(&mut self, name: &str) {
        let address = self.allocate_address();
        self.variable_addresses.insert(name.to_string(), address);
    }

    fn alloc(&mut self) {
        let new_address = self.allocate_address();
        self.stack.push(Value::new_int(new_address));
    }

    fn allocate_address(&mut self) -> i64 {
        if let Some(address) = self.free_list.pop() {
            address
        } else {
            let address = self.next_address;
            self.next_address += 1;
            address
        }
    }

    fn store(&mut self) {
        let addr = self.stack.pop().unwrap();
        match addr {
            Value::Int(address) => {
                let value = self.stack.pop().unwrap();
                self.values_by_address.insert(address, value);
            }
            _ => panic!("Store operation requires an integer address"),
        }
    }

    fn deref(&mut self) {
        let value = self.stack.pop().unwrap();
        match value {
            Value::Int(address) => {
                if let Some(stored_value) = self.values_by_address.get(&address) {
                    self.stack.push(stored_value.clone());
                } else {
                    self.stack.push(Value::Null);
                }
            }
            _ => panic!("Dereference operation requires an integer address"),
        }
    }

    fn mul_deref(&mut self) {
        let address = self.stack.pop().unwrap();
        let n = self.stack.pop().unwrap().extract_int().unwrap();
        if n == 0 {
            self.stack.push(address.clone());
        } else {
            let mut address_p = address.extract_int().unwrap();
            for _ in 0..n {
                address_p = self.values_by_address[&address_p].extract_int().unwrap();
            }
            self.stack.push(self.values_by_address[&address_p].clone());
        }
    }

    fn dup(&mut self) {
        if let Some(value) = self.stack.last().cloned() {
            self.stack.push(value.clone());
            self.stack.push(value);
        } else {
            panic!("Stack underflow on dup operation");
        }
    }

    fn jump_if_false(&mut self, addr: usize) {
        let condition = self.stack.pop().unwrap();
        if !self.is_truthy(condition) {
            self.pc = addr;
        }
    }

    fn call_builtin(&mut self, name: &str, argc: usize) {
        let mut args = Vec::new();
        for _ in 0..argc {
            args.push(self.stack.pop().unwrap());
        }
        args.reverse();
        if let Some(func) = self.builtins.get(name) {
            let result = func(self, args);
            self.stack.push(result);
        } else {
            panic!("Undefined function: {}", name);
        }
    }

    fn free_addr(&mut self) {
        let address = self.stack.pop().unwrap();
        if let Value::Int(addr) = address {
            self.values_by_address.remove(&addr);
            self.free_list.push(addr);
        } else {
            panic!("FreeAddr operation requires an integer address on the stack");
        }
    }

    fn binary_op<F>(&mut self, op: F)
    where
        F: Fn(i64, i64) -> i64,
    {
        let rhs = self.stack.pop().unwrap();
        let lhs = self.stack.pop().unwrap();
        let result = self.perform_arithmetic_op(lhs, rhs, op);
        self.stack.push(result);
    }

    fn binary_op_bool<F>(&mut self, op: F)
    where
        F: Fn(bool, bool) -> bool,
    {
        let rhs = self.stack.pop().unwrap();
        let lhs = self.stack.pop().unwrap();
        let result = self.perform_boolean_op(lhs, rhs, op);
        self.stack.push(result);
    }

    fn binary_op_cmp<F>(&mut self, op: F)
    where
        F: Fn(i64, i64) -> bool,
    {
        let rhs = self.stack.pop().unwrap();
        let lhs = self.stack.pop().unwrap();
        let result = self.perform_comparison_op(lhs, rhs, op);
        self.stack.push(result);
    }

    fn unary_op<F>(&mut self, op: F)
    where
        F: Fn(i64) -> i64,
    {
        let value = self.stack.pop().unwrap();
        let result = self.perform_unary_op(value, op);
        self.stack.push(result);
    }

    fn unary_op_bool<F>(&mut self, op: F)
    where
        F: Fn(bool) -> bool,
    {
        let value = self.stack.pop().unwrap();
        let result = self.perform_unary_bool_op(value, op);
        self.stack.push(result);
    }

    fn perform_arithmetic_op<F>(&self, lhs: Value, rhs: Value, op: F) -> Value
    where
        F: Fn(i64, i64) -> i64,
    {
        match (lhs, rhs) {
            (Value::Int(a), Value::Int(b)) => Value::Int(op(a, b)),
            _ => panic!("Invalid types for arithmetic operation"),
        }
    }

    fn perform_boolean_op<F>(&self, lhs: Value, rhs: Value, op: F) -> Value
    where
        F: Fn(bool, bool) -> bool,
    {
        match (lhs, rhs) {
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(op(a, b)),
            _ => panic!("Invalid types for boolean operation"),
        }
    }

    fn perform_comparison_op<F>(&self, lhs: Value, rhs: Value, op: F) -> Value
    where
        F: Fn(i64, i64) -> bool,
    {
        match (lhs, rhs) {
            (Value::Int(a), Value::Int(b)) => Value::Bool(op(a, b)),
            _ => panic!("Invalid types for comparison operation"),
        }
    }

    fn perform_unary_op<F>(&self, value: Value, op: F) -> Value
    where
        F: Fn(i64) -> i64,
    {
        match value {
            Value::Int(a) => Value::Int(op(a)),
            _ => panic!("Invalid type for unary operation"),
        }
    }

    fn perform_unary_bool_op<F>(&self, value: Value, op: F) -> Value
    where
        F: Fn(bool) -> bool,
    {
        match value {
            Value::Bool(a) => Value::Bool(op(a)),
            _ => panic!("Invalid type for unary boolean operation"),
        }
    }

    fn is_truthy(&self, value: Value) -> bool {
        match value {
            Value::Bool(b) => b,
            _ => panic!("Invalid type for truthy check"),
        }
    }
}
