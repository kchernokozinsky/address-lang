use crate::ast::*;
use std::collections::HashMap;

pub struct Environment {
    function_space: HashMap<String, Value>,
    variable_to_address: HashMap<String, i64>,
    address_to_value: HashMap<i64, Value>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            function_space: HashMap::new(),
            variable_to_address: HashMap::new(),
            address_to_value: HashMap::new(),
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

    pub fn get_value_by_address(&self, address: i64, value: Value) -> Result<&Value, String> {
        match self.address_to_value.get(&address) {
            Some(v) => Ok(v),
            None => return Err(format!("Address '{}' is empty", address)),
        }
    }
}

pub fn eval_algorithm(
    env: &mut Environment,
    Algorithm::Body { statements }: Algorithm,
) -> Result<(), String> {
    for statement in statements {
        if let Err(e) = eval_statement(env, statement) {
            return Err(e);
        }
    }
    Ok(())
}

fn eval_statement(env: &mut Environment, statement: Statement) -> Result<(), String> {
    match statement {
        Statement::Expression { expression } => {
            if let Err(e) = eval_expression(env, expression) {
                return Err(e);
            }

            Ok(())
        }

        Statement::Declare { lhs, rhs, dt } => {
            let address = match eval_expression(env, rhs.clone()) {
                Ok(Value::Int { value }) => value,
                _ => return Err(format!("Expression '{:?}' is not an address", rhs)),
            };

            bind(env, &lhs, address)
        }

        _ => Err(format!("unhandled statement: {:?}", statement)),
    }
}

fn bind(env: &mut Environment, lhs: &Expression, address: i64) -> Result<(), String> {
    match lhs {
        Expression::Var { name } => Ok(env.add_variable(name, address)),
        _ => Err(format!("{:?} is not a variable", lhs)),
    }
}

fn eval_expression(env: &mut Environment, expression: Expression) -> Result<Value, String> {
    match expression {
        Expression::Int { value } => Ok(Value::Int { value }),
        Expression::Call { function, args } => {
            let mut vals = vec![];

            for arg in args {
                match eval_expression(env, *arg) {
                    Ok(value) => vals.push(value),
                    Err(e) => return Err(e),
                }
            }

            let v = match env.get_function(&function) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

            if let Value::Function { function } = v {
                function(vals)
            } else {
                Err(format!("'{}' isn`t  function", &function))
            }
        }

        Expression::Var { name } => env.get_variable(&name),
        _ => Err(format!("unhandled expression: {:?}", expression)),
    }
}

#[derive(Debug)]
pub enum Value {
    NIL,
    Int {
        value: i64,
    },
    Function {
        function: fn(Vec<Value>) -> Result<Value, String>,
    },
}
