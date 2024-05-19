use std::collections::HashMap;

use codegen::bytecode::Bytecode;
use value::Value;

pub struct VM {
    bytecode: Vec<Bytecode>,
    pc: usize,
    stack: Vec<Value>,
    variable_addresses: HashMap<String, i64>,
    values_by_address: HashMap<i64, Value>,
}

impl VM {
    pub fn new(bytecode: Vec<Bytecode>) -> Self {
        Self {
            bytecode,
            pc: 0,
            stack: Vec::new(),
            variable_addresses: HashMap::new(),
            values_by_address: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        while self.pc < self.bytecode.len() {
            let instruction = self.bytecode[self.pc].clone();
            self.pc += 1;
            println!("---{:?}---", self.pc);
            println!("{:?}", instruction);
            
            match instruction {
                Bytecode::Constant(value) => self.stack.push(value),
                Bytecode::GetVar(name) => {
                    let address =  match self.variable_addresses.get(&name) {
                        Some(address) => address.clone(),
                        None => {
                            let address = self.values_by_address.len() as i64;
                            self.variable_addresses.insert(name, address);
                            address
                        },
                    };
                    self.stack.push(Value::Int(address));
                }
                Bytecode::SetVar(name) => {
                    let address = self.values_by_address.len() as i64;
                    self.variable_addresses.insert(name, address);
                }
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
                Bytecode::Less => {println!("{:?}", self.values_by_address[&0]);self.binary_op_cmp(|a, b| a < b)},
                Bytecode::Not => self.unary_op_bool(|a| !a),
                Bytecode::Negate => self.unary_op(|a| -a),
                Bytecode::Jump(addr) => self.pc = addr,
                Bytecode::JumpIfFalse(addr) => {
                    let condition = self.stack.pop().unwrap();
                    if !self.is_truthy(condition) {
                        self.pc = addr;
                    }
                }
                Bytecode::Label(_) => {}
                Bytecode::Call(_, _) => unimplemented!(),
                Bytecode::Return => return,
                Bytecode::Halt => break,
                Bytecode::Pop => {
                    self.stack.pop();
                }
                Bytecode::Send => unimplemented!(),
                Bytecode::Deref => {
                    let value = self.stack.pop().unwrap();
                    println!("val: {:?}", value);
                    println!("hashmap: {:?}", self.values_by_address);
                    match value {
                        Value::Null => todo!(),
                        Value::Float(_) => todo!(),
                        Value::String(_) => todo!(),
                        Value::Bool(_) => todo!(),
                        Value::Int(address) => self.stack.push(self.values_by_address[&address].clone()),
                        Value::Function(_) => todo!(),
                    }
                },
                Bytecode::MulDeref => unimplemented!(),
                Bytecode::Alloc => {
                    let addr = self.stack.pop().unwrap();
                    

                    match addr {
                        Value::Null => todo!(),
                        Value::Float(_) => todo!(),
                        Value::String(_) => todo!(),
                        Value::Bool(_) => todo!(),
                        Value::Int(address) => {
                            let value = self.stack.pop().unwrap();
                            self.values_by_address.insert(address, value);
                        },
                        Value::Function(_) => todo!(),
                    }
                },
                Bytecode::CallProc(_) => todo!(),
                Bytecode::CallFn(_) => todo!(),
            }
            println!("stack: {:?}", self.stack);
        }
        // print!("stackL: {:?}", self.stack);
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

mod tests {
    use super::*;

    #[test]
    fn test_vm_execution() {
        let bytecode = vec![
            Bytecode::Constant(Value::new_int(5)),
            Bytecode::SetVar("x".to_string()),
            Bytecode::GetVar("x".to_string()),
            Bytecode::Constant(Value::new_int(3)),
            Bytecode::Add,
            Bytecode::SetVar("y".to_string()),
            Bytecode::GetVar("y".to_string()),
            Bytecode::Halt,
        ];

        let mut vm = VM::new(bytecode);
        vm.run();

        assert_eq!(vm.stack.pop().unwrap(), Value::new_int(8));
    }
}
