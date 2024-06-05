pub mod error;

use error::VMError;
use log::trace;
use std::collections::HashMap;
use codegen::bytecode::Bytecode;
use value::{error::ValueError, Value};

use crate::{builtins::BuiltinFunction, heap::Heap, scope::Scope};


pub struct VM {
    bytecode: Vec<Bytecode>,
    pc: usize,
    stack: Vec<Value>,
    scopes: Vec<Scope>,
    heap: Heap,
    builtins: HashMap<String, BuiltinFunction>,
    call_stack: Vec<usize>,
}

impl VM {
    pub fn new(bytecode: Vec<Bytecode>) -> Self {
        Self {
            bytecode,
            pc: 0,
            stack: Vec::new(),
            scopes: vec![Scope::new()],
            heap: Heap::new(),
            builtins: HashMap::new(),
            call_stack: Vec::new(),
        }
    }

    pub fn register_builtin(&mut self, name: &str, func: BuiltinFunction) {
        self.builtins.insert(name.to_string(), func);
    }

    pub fn run(&mut self) -> Result<(), VMError> {
        while self.pc < self.bytecode.len() {
            let instruction = self.bytecode[self.pc].clone();
            self.pc += 1;

            trace!("--- PC: {:?} ---", self.pc);
            trace!("Instruction: {:?}", instruction);

            match instruction {
                Bytecode::Constant(value) => self.stack.push(value),
                Bytecode::LoadVar(name) => self.get_var(&name)?,
                Bytecode::StoreVar(name) => self.set_var(&name)?,
                Bytecode::Add => self.binary_op(Value::sum)?,
                Bytecode::Sub => self.binary_op(Value::sub)?,
                Bytecode::Mul => self.binary_op(Value::mul)?,
                Bytecode::Div => self.binary_op(Value::div)?,
                Bytecode::Mod => self.binary_op(Value::modulus)?,
                Bytecode::And => self.binary_op(Value::and)?,
                Bytecode::Or => self.binary_op(Value::or)?,
                Bytecode::Equal => self.binary_op(Value::eq)?,
                Bytecode::NotEqual => self.binary_op(Value::ne)?,
                Bytecode::Greater => self.binary_op(Value::gt)?,
                Bytecode::Less => self.binary_op(Value::lt)?,
                Bytecode::Not => self.unary_op(Value::not)?,
                Bytecode::Negate => self.unary_op(Value::negate)?,
                Bytecode::Jump(addr) => self.pc = addr,
                Bytecode::JumpIfFalse(addr) => self.jump_if_false(addr)?,
                Bytecode::Label(_) => {}
                Bytecode::CallBuiltin(name, argc) => self.call_builtin(&name, argc)?,
                Bytecode::CallSubProgram(label, argc) => {
                    self.call_subprogram(label, argc);
                }
                Bytecode::Return => self.handle_return()?,
                Bytecode::Halt => break,
                Bytecode::Pop => {
                    self.stack.pop().ok_or(VMError::StackUnderflow)?;
                }
                Bytecode::Deref => self.deref()?,
                Bytecode::MulDeref => self.mul_deref()?,
                Bytecode::Store => self.store()?,
                Bytecode::Alloc => self.alloc()?,
                Bytecode::AllocMany(count) => self.alloc_many(count)?,
                Bytecode::Dup => self.dup()?,
                Bytecode::StoreAddr => self.store_addr()?,
                Bytecode::BindAddr(name) => self.bind_addr(name)?,
                Bytecode::PushScope => self.push_scope(),
                Bytecode::PopScope => self.pop_scope()?,
                Bytecode::FreeAddr => self.free_addr()?,
                Bytecode::Swap => self.swap()?,
            }

            trace!("Stack: {:?}", self.stack);
            trace!("Values by address: {:?}", self.heap);
            trace!("Current scope: {:?}", self.current_scope());
        }
        Ok(())
    }

    fn swap(&mut self) -> Result<(), VMError> {
        if self.stack.len() < 2 {
            return Err(VMError::StackUnderflow);
        }
        let len = self.stack.len();
        self.stack.swap(len - 1, len - 2);
        Ok(())
    }

    fn current_scope(&mut self) -> &mut Scope {
        self.scopes.last_mut().expect("No scope available")
    }

    fn push_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    fn pop_scope(&mut self) -> Result<(), VMError> {
        self.scopes
            .pop()
            .ok_or(VMError::Custom("No scope to pop".to_string()))?;
        Ok(())
    }

    fn store(&mut self) -> Result<(), VMError> {
        let addr = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        match addr {
            Value::Int(address) => {
                let value = self.stack.pop().ok_or(VMError::StackUnderflow)?;
                self.heap.store(address, value);
                Ok(())
            }
            _ => Err(VMError::InvalidAddress),
        }
    }

    fn store_addr(&mut self) -> Result<(), VMError> {
        let value = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        self.stack
            .push(Value::new_int(self.heap.store_value(value)));
        Ok(())
    }

    fn bind_addr(&mut self, name: String) -> Result<(), VMError> {
        let address = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        if let Value::Int(addr) = address {
            self.current_scope().set_var(&name, addr);
            Ok(())
        } else {
            Err(VMError::InvalidAddress)
        }
    }

    fn get_var(&mut self, name: &str) -> Result<(), VMError> {
        let address = self.current_scope().get_var(name).unwrap_or_else(|| {
            let address = self.heap.allocate_address();
            self.current_scope().set_var(name, address);
            address
        });
        self.stack.push(Value::Int(address));
        Ok(())
    }

    fn set_var(&mut self, name: &str) -> Result<(), VMError> {
        let address = self.heap.allocate_address();
        self.current_scope().set_var(name, address);
        Ok(())
    }

    fn alloc(&mut self) -> Result<(), VMError> {
        let new_address = self.heap.allocate_address();
        self.stack.push(Value::new_int(new_address));
        Ok(())
    }

    fn alloc_many(&mut self, count: usize) -> Result<(), VMError> {
        let addresses = self.heap.allocate_consecutive_addresses(count);
        for address in addresses.iter().copied() {
            self.stack.push(Value::new_int(address));
        }
        Ok(())
    }

    fn deref(&mut self) -> Result<(), VMError> {
        let value = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        match value {
            Value::Int(address) => {
                if let Some(stored_value) = self.heap.lookup_address(address) {
                    self.stack.push(stored_value.clone());
                } else {
                    self.stack.push(Value::Null);
                }
                Ok(())
            }
            _ => Err(VMError::InvalidAddress),
        }
    }

    fn allocate_list(&mut self, elements: Vec<Value>) -> Result<i64, VMError> {
        let mut addresses = self.heap.allocate_consecutive_addresses(2);
        let head = addresses[0];
        let mut i = 0;
        for elem in elements.clone() {
            i += 1;
            self.store()?;
            self.heap.store(addresses[0], Value::Null);
            self.heap.store(addresses[1], elem);

            if i != elements.len() {
                let next: Vec<i64> = self.heap.allocate_consecutive_addresses(2);
                self.heap.store(addresses[0], Value::new_int(next[0]));
                addresses = next;
            }
        }
        Ok(head)
    }

    fn mul_deref(&mut self) -> Result<(), VMError> {
        let n: i64 = self
            .stack
            .pop()
            .ok_or(VMError::StackUnderflow)?
            .extract_int()?;
        let address = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        if n == 0 {
            self.stack.push(address.clone());
        } else if n < 0 {
            let values = vec![address];
            let fathers = self.mul_minus_deref_vec(values, n)?;
            if fathers.is_empty() {
                self.stack.push(Value::Null);
            } else {
                let head = self.allocate_list(fathers)?;
                self.stack.push(Value::Int(head));
            }
        } else {
            let mut address_p = address.extract_int()?;
            for _ in 1..n {
                address_p = self
                    .heap
                    .lookup_address(address_p)
                    .ok_or(VMError::InvalidAddress)?
                    .extract_int()?;
            }

            self.stack.push(
                self.heap
                    .lookup_address(address_p)
                    .ok_or(VMError::InvalidAddress)?
                    .clone(),
            );
        }
        Ok(())
    }

    fn mul_minus_deref_vec(&self, values: Vec<Value>, n: i64) -> Result<Vec<Value>, VMError> {
        if n == 0 {
            return Ok(values);
        }
        let selected_ids: Vec<Value> = self.heap.lookup_values(values);
        self.mul_minus_deref_vec(selected_ids, n + 1)
    }

    fn dup(&mut self) -> Result<(), VMError> {
        if let Some(value) = self.stack.last().cloned() {
            self.stack.push(value.clone());
            self.stack.push(value);
            Ok(())
        } else {
            Err(VMError::StackUnderflow)
        }
    }

    fn jump_if_false(&mut self, addr: usize) -> Result<(), VMError> {
        let condition = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        if !self.is_truthy(condition)? {
            self.pc = addr;
        }
        Ok(())
    }

    fn call_builtin(&mut self, name: &str, argc: usize) -> Result<(), VMError> {
        let mut args = Vec::new();
        for _ in 0..argc {
            args.push(self.stack.pop().ok_or(VMError::StackUnderflow)?);
        }
        args.reverse();
        if let Some(func) = self.builtins.get(name) {
            let result = func(self, args);
            self.stack.push(result);
            Ok(())
        } else {
            Err(VMError::UndefinedFunction(name.to_string()))
        }
    }

    fn call_subprogram(&mut self, label: usize, argc: usize) {
        self.call_stack.push(self.pc + 1);
        self.pc = label;
    }

    fn handle_return(&mut self) -> Result<(), VMError> {
        self.pop_scope()?;
        self.pc = self.call_stack.pop().ok_or(VMError::StackUnderflow)?;
        Ok(())
    }

    fn free_addr(&mut self) -> Result<(), VMError> {
        let address = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        if let Value::Int(addr) = address {
            self.heap.free(addr);
            Ok(())
        } else {
            Err(VMError::InvalidAddress)
        }
    }

    fn binary_op<F>(&mut self, op: F) -> Result<(), VMError>
    where
        F: Fn(&Value, &Value) -> Result<Value, ValueError>,
    {
        let rhs = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        let lhs = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        let result = op(&lhs, &rhs)?;
        self.stack.push(result);
        Ok(())
    }

    fn unary_op<F>(&mut self, op: F) -> Result<(), VMError>
    where
        F: Fn(&Value) -> Result<Value, ValueError>,
    {
        let val = self.stack.pop().ok_or(VMError::StackUnderflow)?;
        let result = op(&val)?;
        self.stack.push(result);
        Ok(())
    }

    fn is_truthy(&self, value: Value) -> Result<bool, VMError> {
        match value {
            Value::Bool(b) => Ok(b),
            _ => Err(VMError::InvalidOperation),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm_execution() {
        let bytecode = vec![
            Bytecode::Constant(Value::new_int(5)),
            Bytecode::BindAddr("x".to_string()),
            Bytecode::LoadVar("x".to_string()),
            Bytecode::Constant(Value::new_int(3)),
            Bytecode::Add,
            Bytecode::BindAddr("y".to_string()),
            Bytecode::LoadVar("y".to_string()),
            Bytecode::Halt,
        ];

        let mut vm = VM::new(bytecode);
        assert!(vm.run().is_ok());
        assert_eq!(vm.stack.pop().unwrap(), Value::new_int(8));
    }

    #[test]
    fn test_alloc() {
        let bytecode = vec![Bytecode::Alloc, Bytecode::Halt];

        let mut vm = VM::new(bytecode);
        assert!(vm.run().is_ok());
        assert_eq!(vm.stack.pop().unwrap(), Value::new_int(0));
    }

    #[test]
    fn test_free_addr() {
        let bytecode = vec![
            Bytecode::Constant(Value::new_int(10)),
            Bytecode::StoreAddr,
            Bytecode::FreeAddr,
            Bytecode::Alloc,
            Bytecode::Halt,
        ];

        let mut vm = VM::new(bytecode);
        assert!(vm.run().is_ok());
        assert_eq!(vm.stack.pop().unwrap(), Value::new_int(0));
    }

    #[test]
    fn test_deref_unallocated_address() {
        let bytecode = vec![
            Bytecode::Constant(Value::new_int(42)),
            Bytecode::StoreAddr,
            Bytecode::Constant(Value::new_int(100)),
            Bytecode::Deref,
            Bytecode::Halt,
        ];

        let mut vm = VM::new(bytecode);
        assert!(vm.run().is_ok());
        assert_eq!(vm.stack.pop().unwrap(), Value::Null);
    }
}
