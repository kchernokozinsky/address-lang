use crate::ast::*;
pub mod builtins;
pub mod environment;
pub mod value;
use environment::*;
use value::*;

pub struct Compiler {
    lines: Vec<FileLine>,
    env: Environment,
    current: usize,
}

impl Compiler {
    pub fn new(lines: Vec<FileLine>, env: Environment) -> Compiler {
        let mut compiler = Compiler { lines, env, current: 0 };
        compiler.extract_labels();
        compiler
    }

    pub fn increment_line(&mut self) {
        self.current += 1;
    }

    fn extract_labels(&mut self) {
        for (index, line) in self.lines.iter().enumerate() {
            let labels = line.labels(); 
                for label in labels {
                    self.env.add_label(label.to_string(), index);
                }
            
        }
    }

    pub fn compile(&mut self) -> Result<(), String> {
        eval_algorithm(self)
    }
}

fn eval_algorithm(cmp: &mut Compiler) -> Result<(), String> {
    loop {
        let cur = cmp.current;
        let line: FileLine = cmp.lines[cur].clone();

        if let Err(e) = eval_file_line(cmp, line) {
            return Err(e);
        }
        if cur == cmp.current {
            cmp.current += 1;
            if cmp.current >= cmp.lines.len() {
                break;
            }
        }
    }
    Ok(())
}

fn eval_file_line(cmp: &mut Compiler, line: FileLine) -> Result<(), String> {
    match line {
        FileLine::Line {
            labels: _s,
            statements,
        } => {
            for statement in statements {
                if let Err(e) = eval_statement(cmp, statement) {
                    return Err(e);
                }
            }
            Ok(())
        }
        FileLine::FormulaLine {
            labels: _,
            statement: _,
        } => Ok(()),
    }
}

fn eval_statement(cmp: &mut Compiler, statement: Statement) -> Result<(), String> {
    match statement {
        Statement::Expression { expression } => {
            if let Err(e) = eval_expression(cmp, expression) {
                return Err(e);
            }

            Ok(())
        }

        Statement::Assign { lhs, rhs } => {
            let address = match eval_expression(cmp, rhs.clone()) {
                Ok(Value::Int { value }) => value,
                _ => return Err(format!("Expression '{:?}' is not an address", rhs)),
            };

            bind(cmp, &lhs, address)
        }

        Statement::UnconditionalJump { label } => match cmp.env.get_line_by_label(&label) {
            Some(line) => {
                cmp.current = line.clone();
                Ok(())
            }
            None => {
                return Err(format!(
                    "You tried to jump to '{:?}' which is not declared",
                    label
                ))
            }
        },

        Statement::Predicate {
            condition,
            if_true,
            if_false,
        } => {
            let cond = match eval_expression(cmp, condition) {
                Ok(value) => match value {
                    Value::Bool { value } => value,
                    v => return Err(format!("Value '{:?}' is not a boolean", v)),
                },
                Err(e) => return Err(e),
            };
            if cond {
                eval_statement(cmp, *if_true)
            } else {
                eval_statement(cmp, *if_false)
            }
        }

        Statement::Send { lhs, rhs } => {
            let address = match eval_expression(cmp, lhs.clone()) {
                Ok(Value::Int { value }) => value,
                _ => return Err(format!("Expression '{:?}' is not an address", rhs)),
            };

            let value = match eval_expression(cmp, rhs.clone()) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

            Ok(cmp.env.fill_address(address, value))
        }

        _ => Err(format!("unhandled statement: {:?}", statement)),
    }
}

fn bind(cmp: &mut Compiler, lhs: &Expression, address: i64) -> Result<(), String> {
    match lhs {
        Expression::Var { name } => Ok(cmp.env.add_variable(name, address)),
        _ => Err(format!("{:?} is not a variable", lhs)),
    }
}

fn eval_expression(cmp: &mut Compiler, expression: Expression) -> Result<Value, String> {

    match expression {
        Expression::Int { value } => Ok(Value::Int { value }),
        Expression::Call { function, args } => {
            let mut vals = vec![];

            for arg in args {
                match eval_expression(cmp, *arg) {
                    Ok(value) => vals.push(value),
                    Err(e) => return Err(e),
                }
            }

            let v = match cmp.env.get_function(&function) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

            if let Value::Function { function } = v {
                function(vals)
            } else {
                Err(format!("'{}' isn`t  function", &function))
            }
        }

        Expression::BinaryOp { op, lhs, rhs } => {
            let lv = match eval_expression(cmp, *lhs) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

            let rv = match eval_expression(cmp, *rhs) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

            match op {
                BinaryOp::Sum => Value::sum(lv, rv),
                BinaryOp::Sub => Value::sub(lv, rv),
                BinaryOp::EQ => Ok(Value::Bool { value: lv.eq(&rv) }),
                BinaryOp::NE => Ok(Value::Bool { value: lv.ne(&rv) }),
                BinaryOp::LT => Ok(Value::Bool { value: lv.lt(&rv) }),
                _ => Err(format!("operator {:?} is unhandled", op)),
            }
        }

        Expression::Var { name } => cmp.env.get_variable(&name),

        Expression::UnaryOp { op, expr } => match op {
            UnaryOp::Dereference => match eval_expression(cmp, *expr.clone()) {
                Ok(Value::Int { value }) => match cmp.env.get_value_by_address(value) {
                    Ok(value) => Ok(value.clone()),
                    _ => Ok(Value::Null),
                },
                _ => return Err(format!("Expression '{:?}' is not an address", expr)),
            },
            UnaryOp::Not => {
                let val = match eval_expression(cmp, *expr.clone()) {
                    Ok(value) => match value {
                        Value::Bool { value } => return Ok(Value::Bool { value: !value}),
                        v => return Err(format!("Value '{:?}' is not a boolean", v)),
                    },
                    Err(e) => return Err(e),
                };
            }
            _ => Err(format!("operator {:?} is unhandled", op)),
        },
        Expression::Bool { value } => Ok(Value::Bool { value }),
    }
}
