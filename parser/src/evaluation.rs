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

    pub fn get_value_by_address(&self, address: i64) -> Result<&Value, String> {
        match self.address_to_value.get(&address) {
            Some(v) => Ok(v),
            None => return Err(format!("Address '{}' is empty", address)),
        }
    }
}

pub fn eval_algorithm(
    env: &mut Environment,
    Algorithm::Body { lines }: Algorithm,
) -> Result<(), String> {
    for line in lines {
        if let Err(e) = eval_file_line(env, line) {
            return Err(e);
        }
    }
    Ok(())
}


fn eval_file_line( env: &mut Environment, line: FileLine ) -> Result<(), String> {
        match line {
            FileLine::Line { statements } => {
                for statement in statements {
                    if let Err(e) = eval_statement(env, statement) {
                        return Err(e);
                    }
                }
                Ok(())
            }
            FileLine::LabeledLine { labels } => {
                Ok(())
            },
        }

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

        Statement::Assign { lhs, rhs } => {
            let address = match eval_expression(env, lhs.clone()) {
                Ok(Value::Int { value }) => value,
                _ => return Err(format!("Expression '{:?}' is not an address", rhs)),
            };

            let value = match eval_expression(env, rhs.clone()) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

            Ok(env.fill_address(address, value))
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
        },

        Expression::BinaryOp { op, lhs, rhs } => {
            let lv = match eval_expression(env, *lhs) {
                Ok(v) => v,
                Err(e) => return  Err(e),
            };

            let rv = match eval_expression(env, *rhs) {
                Ok(v) => v,
                Err(e) => return  Err(e),
            };


            match op  {
                BinaryOp::Sum => Value::sum(lv, rv),
                BinaryOp::Sub => Value::sub(lv, rv),
                _ => Err(format!("operator {:?} is unhandled", op)) 
            }

        }

        Expression::Var { name } => env.get_variable(&name),

        Expression::UnaryOp { op, expr } => {
            match op {
                UnaryOp::Dereference => match eval_expression(env, *expr.clone())
                {
                    Ok(Value::Int { value }) => match env.get_value_by_address(value) {
                        Ok(value) => Ok(value.clone()),
                        _ => Ok(Value::NIL)
                    },
                    _ => return Err(format!("Expression '{:?}' is not an address", expr)),
                },
                _ => Err(format!("operator {:?} is unhandled", op)),
            }
        },

        _ => Err(format!("unhandled expression: {:?}", expression)),
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    NIL,
    Int {
        value: i64,
    },
    Function {
        function: fn(Vec<Value>) -> Result<Value, String>,
    },
}
impl Value {
    fn sum(lv: Value, rv: Value) -> Result<Value, String> {
        let lv_ = match lv {
            Value::Int { value } => value,
            _ => return Err(format!("{:?} and {:?} are not compatible", lv, rv))
        };

        let rv_ = match rv {
            Value::Int { value } => value,
            _ => return Err(format!("{:?} and {:?} are not compatible", lv, rv))
        };

        Ok(Value::Int{value: lv_ + rv_})
    }

    fn sub(lv: Value, rv: Value) -> Result<Value, String> {
        let lv_ = match lv {
            Value::Int { value } => value,
            _ => return Err(format!("{:?} and {:?} are not compatible", lv, rv))
        };

        let rv_ = match rv {
            Value::Int { value } => value,
            _ => return Err(format!("{:?} and {:?} are not compatible", lv, rv))
        };

        Ok(Value::Int{value: lv_ - rv_})
    }
}