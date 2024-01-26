pub mod builtins;
pub mod environment;
pub mod value;
use crate::ast::*;
use environment::*;
use value::*;

pub struct Evaluator {
    lines: Vec<FileLine>,
    env: Environment,
    current_line: usize,
    to_stop: bool,
    is_jumped: bool,
}

impl Evaluator {
    pub fn new(lines: Vec<FileLine>, env: Environment) -> Evaluator {
        let mut evaluator = Evaluator {
            lines,
            env,
            current_line: 0,
            to_stop: false,
            is_jumped: false,
        };
        evaluator.extract_labels();
        evaluator
    }

    pub fn increment_line(&mut self) {
        self.current_line += 1;
    }

    fn extract_labels(&mut self) {
        for (index, line) in self.lines.iter().enumerate() {
            let labels = line.labels();
            for label in labels {
                self.env.add_label(label.to_string(), index);
            }
        }
    }

    pub fn stop_eval(&mut self) {
        self.to_stop = true
    }

    pub fn eval(&mut self) -> Result<(), String> {
        loop {
            if self.to_stop {
                return Ok(());
            } else {
                let cur = self.current_line;
                let line: FileLine = self.lines[cur].clone();

                if let Err(e) = self.eval_file_line(line) {
                    return Err(e);
                }
                if !self.is_jumped {
                    self.current_line += 1;
                    if self.current_line >= self.lines.len() {
                        break;
                    }
                } else {
                    self.is_jumped = false;
                }
            }
        }
        Ok(())
    }

    fn eval_file_line(&mut self, line: FileLine) -> Result<(), String> {
        match line {
            FileLine::Line {
                labels: _s,
                statements,
            } => self.eval_statements(statements),
        }
    }

    pub fn eval_statements(&mut self, statements: Statements) -> Result<(), String> {
        match statements {
            Statements::OneLineStatement(stmnt) => self.eval_one_line_statement(stmnt),
            Statements::SimpleStatements(stmnts) => {
                for statement in stmnts {
                    if let Err(e) = self.eval_statement(statement) {
                        return Err(e);
                    }
                }
                Ok(())
            }
        }
    }

    fn eval_loop(
        &mut self,
        step: i64,
        last_value_or_condition: Value,
        iterator: i64,
        label_until: &String,
        label_to: &String,
        condition: Option<Expression>,
    ) -> Result<(), String> {
        let line_from = self.current_line + 1;
        let line_to = *self
            .env
            .get_line_by_label(label_until)
            .expect(format!("label '{}' is not declared!", &label_until).as_str());

        let line_to_jump = match self.env.get_line_by_label(label_to) {
            Some(line) => *line,
            None => {
                return Err(format!(
                    "You tried to jump to '{:?}' which is not declared",
                    label_to
                ))
            }
        };
        let mut it_just_set: bool = true;
        self.current_line = line_from;
        loop {
            let mut it_value = match self.env.get_value_by_address(iterator) {
                Ok(v) => match v {
                    Value::Int { value } => *value,
                    _ => return Err(format!("value '{}' is not a valid integer", v)),
                },
                Err(e) => return Err(e),
            };
            // increment iterator
            if !it_just_set {
                it_value = it_value + step;
                self.env
                    .fill_address(iterator, Value::Int { value: it_value });
            } else {
                it_just_set = false
            }
            // check condition

            match last_value_or_condition {
                Value::Bool { .. } => {
                    let _cond: Expression = condition.clone().unwrap();
                    let cond = {
                        match self.eval_expression(_cond) {
                            Ok(v) => match v {
                                Value::Bool { value } => value,
                                _ => {
                                    return Err(format!(
                                        " expression '{:?}' is not a Bool!",
                                        condition.unwrap()
                                    ))
                                }
                            },
                            Err(e) => return Err(e),
                        }
                    };

                    if cond {
                    } else {
                        self.is_jumped = true;
                        self.current_line = line_to_jump;
                        return Ok(());
                    }
                }

                Value::Int { value } => {
                    if it_value <= value {
                    } else {
                        self.is_jumped = true;
                        self.current_line = line_to_jump;
                        return Ok(());
                    }
                }

                _ => {
                    return Err(format!(
                        "Expression '{:?}' is not an Integer or Bool",
                        last_value_or_condition
                    ))
                }
            }

            loop {
                if self.to_stop {
                    return Ok(());
                } else {
                    let cur = self.current_line;
                    let line: FileLine = self.lines[cur].clone();

                    if let Err(e) = self.eval_file_line(line) {
                        return Err(e);
                    }
                    if !self.is_jumped {
                        if self.current_line == line_to {
                            self.current_line = line_from;
                            break;
                        } else {
                            self.current_line += 1;
                            if self.current_line >= self.lines.len() {
                                return Ok(());
                            }
                        }
                    } else {
                        if (self.current_line >= line_from) && (self.current_line < line_to) {
                        } else {
                            return Ok(());
                        }
                    }
                }
            }
        }
    }

    fn eval_one_line_statement(&mut self, statement: OneLineStatement) -> Result<(), String> {
        match statement {
            OneLineStatement::Loop {
                initial_value,
                step,
                last_value_or_condition,
                iterator,
                label_until,
                label_to,
            } => {
                let l_it_adrress = match self.eval_expression(iterator.clone()) {
                    Ok(v) => match v {
                        Value::Int { value } => value,
                        _ => {
                            return Err(format!(
                                "Expression '{:?}' is not an address or variable",
                                &iterator
                            ))
                        }
                    },
                    Err(e) => return Err(e),
                };

                let l_init_value = match self.eval_expression(initial_value.clone()) {
                    Ok(v) => match v {
                        Value::Int { value } => value,
                        _ => {
                            return Err(format!(
                                "Expression '{:?}' is not an Integer",
                                &initial_value
                            ))
                        }
                    },
                    Err(e) => return Err(e),
                };

                self.env.fill_address(
                    l_it_adrress,
                    Value::Int {
                        value: l_init_value,
                    },
                );

                let l_step = match self.eval_expression(step.clone()) {
                    Ok(v) => match v {
                        Value::Int { value } => value,
                        _ => return Err(format!("Expression '{:?}' is not an Integer", &step)),
                    },
                    Err(e) => return Err(e),
                };

                let mut cond = None;
                let l_last_value_or_condition: Value =
                    match self.eval_expression(last_value_or_condition.clone()) {
                        Ok(v) => match v {
                            Value::Int { value } => Value::Int { value },
                            Value::Bool { value } => {
                                cond = Some(last_value_or_condition.clone());
                                Value::Bool { value }
                            }
                            _ => {
                                return Err(format!(
                                    "Expression '{:?}' is not an Integer or Bool",
                                    &step
                                ))
                            }
                        },
                        Err(e) => return Err(e),
                    };

                self.eval_loop(
                    l_step,
                    l_last_value_or_condition,
                    l_it_adrress,
                    &label_until,
                    &label_to,
                    cond,
                )
            }
            OneLineStatement::Predicate {
                condition,
                if_true,
                if_false,
            } => {
                let cond = match self.eval_expression(condition) {
                    Ok(value) => match value {
                        Value::Bool { value } => value,
                        v => return Err(format!("Value '{:?}' is not a boolean", v)),
                    },
                    Err(e) => return Err(e),
                };
                if cond {
                    self.eval_statements(*if_true)
                } else {
                    self.eval_statements(*if_false)
                }
            }
            OneLineStatement::Exit => Ok(self.stop_eval()),
            OneLineStatement::UnconditionalJump { label } => {
                match self.env.get_line_by_label(&label) {
                    Some(line) => {
                        self.is_jumped = true;
                        self.current_line = line.clone();
                        Ok(())
                    }
                    None => {
                        return Err(format!(
                            "You tried to jump to '{:?}' which is not declared",
                            label
                        ))
                    }
                }
            }
            OneLineStatement::SubProgram {
                sp_name,
                args,
                label_to,
            } => Ok(()),
            OneLineStatement::Return => Ok(()),
        }
    }

    fn eval_statement(&mut self, statement: SimpleStatement) -> Result<(), String> {
        match statement {
            SimpleStatement::Expression { expression } => {
                if let Err(e) = self.eval_expression(expression) {
                    return Err(e);
                }

                Ok(())
            }

            SimpleStatement::Assign { lhs, rhs } => {
                let r = match self.eval_expression(rhs.clone()) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                };
                let l = match lhs.clone() {
                    Expression::Null => todo!(),
                    Expression::Float { .. } => {
                        return Err(format!(
                            "Expression '{:?}' is not an address or variable",
                            lhs
                        ))
                    }
                    Expression::Bool { .. } => {
                        return Err(format!(
                            "Expression '{:?}' is not an address or variable",
                            lhs
                        ))
                    }
                    Expression::Int { .. } => self.eval_expression(lhs.clone())?,
                    Expression::String { .. } => {
                        return Err(format!(
                            "Expression '{:?}' is not an address or variable",
                            lhs
                        ))
                    }
                    Expression::Var { .. } => {
                        if let Value::Int { value } = r {
                            return self.bind(&lhs, value);
                        } else {
                            return Err(format!("Expression '{:?}' is not an address", rhs));
                        }
                    }
                    Expression::Call { .. } => self.eval_expression(lhs.clone())?,
                    Expression::UnaryOp { op, expr } => match op {
                        UnaryOp::Dereference => self.eval_expression(*expr)?,
                        UnaryOp::Not => self.eval_expression(lhs.clone())?,
                    },
                    Expression::BinaryOp { .. } => self.eval_expression(lhs.clone())?,
                };

                match l {
                    Value::Int { value } => Ok(self.env.fill_address(value, r)),
                    _ => {
                        return Err(format!(
                            "Expression '{:?}' is not an address or variable",
                            l
                        ))
                    }
                }
            }

            SimpleStatement::Send { lhs, rhs } => {
                let address = match self.eval_expression(lhs.clone()) {
                    Ok(Value::Int { value }) => value,
                    _ => return Err(format!("Expression '{:?}' is not an address", rhs)),
                };

                let value = match self.eval_expression(rhs.clone()) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                };

                Ok(self.env.fill_address(address, value))
            }

            _ => Err(format!("unhandled statement: {:?}", statement)),
        }
    }

    fn bind(&mut self, lhs: &Expression, address: i64) -> Result<(), String> {
        match lhs {
            Expression::Var { name } => Ok(self.env.add_variable(name, address)),
            _ => Err(format!("{:?} is not a variable", lhs)),
        }
    }

    fn eval_expression(&mut self, expression: Expression) -> Result<Value, String> {
        match expression {
            Expression::Int { value } => Ok(Value::Int { value }),
            Expression::Call { function, args } => {
                let mut vals = vec![];

                for arg in args {
                    match self.eval_expression(*arg) {
                        Ok(value) => vals.push(value),
                        Err(e) => return Err(e),
                    }
                }

                let v = match self.env.get_function(&function) {
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
                let lv = match self.eval_expression(*lhs) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                };

                let rv = match self.eval_expression(*rhs) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                };

                match op {
                    BinaryOp::Sum => Value::sum(lv, rv),
                    BinaryOp::Sub => Value::sub(lv, rv),
                    BinaryOp::Mul => Value::mul(lv, rv),
                    BinaryOp::EQ => Ok(Value::Bool { value: lv.eq(&rv) }),
                    BinaryOp::NE => Ok(Value::Bool { value: lv.ne(&rv) }),
                    BinaryOp::LT => Ok(Value::Bool { value: lv.lt(&rv) }),
                    _ => Err(format!("operator {:?} is unhandled", op)),
                }
            }

            Expression::Var { name } => self.env.get_variable(&name),

            Expression::UnaryOp { op, expr } => match op {
                UnaryOp::Dereference => match self.eval_expression(*expr.clone()) {
                    Ok(Value::Int { value }) => match self.env.get_value_by_address(value) {
                        Ok(value) => Ok(value.clone()),
                        _ => Ok(Value::Null),
                    },
                    _ => return Err(format!("Expression '{:?}' is not an address", expr)),
                },
                UnaryOp::Not => {
                    match self.eval_expression(*expr.clone()) {
                        Ok(value) => match value {
                            Value::Bool { value } => return Ok(Value::Bool { value: !value }),
                            v => return Err(format!("Value '{:?}' is not a boolean", v)),
                        },
                        Err(e) => return Err(e),
                    };
                }
            },
            Expression::Bool { value } => Ok(Value::Bool { value }),
            Expression::String { value } => Ok(Value::String { value }),
            Expression::Float { value } => Ok(Value::Float { value }),
            Expression::Null => Ok(Value::Null),
        }
    }
}
