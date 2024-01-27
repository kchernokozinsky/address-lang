pub mod builtins;
pub mod environment;
pub mod errors;
pub mod value;
use crate::ast::*;
use crate::location::Location;
use environment::*;
use errors::*;
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

    pub fn eval(&mut self) -> Result<(), EvaluationError> {
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

    fn eval_file_line(&mut self, line: FileLine) -> Result<(), EvaluationError> {
        match line {
            FileLine::Line {
                labels: _s,
                statements,
            } => self.eval_statements(statements),
        }
    }

    pub fn eval_statements(&mut self, statements: Statements) -> Result<(), EvaluationError> {
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
        l_location: Location,
        r_location: Location,
    ) -> Result<(), EvaluationError> {
        let line_from = self.current_line + 1;
        let line_to = *self
            .env
            .get_line_by_label(label_until)
            .expect(format!("label '{}' is not declared!", &label_until).as_str());

        let line_to_jump = match self.env.get_line_by_label(label_to) {
            Some(line) => *line,
            None => {
                return Err(EvaluationError::RuntimeError(RuntimeError::LabelNotFound(
                    l_location,
                    r_location,
                    label_to.to_string(),
                )));
            }
        };
        let mut it_just_set: bool = true;
        self.current_line = line_from;
        loop {
            let mut it_value = match self.env.get_value_by_address(iterator) {
                Ok(v) => match v {
                    Value::Int { value } => *value,
                    _ => {
                        return Err(EvaluationError::RuntimeError(RuntimeError::TypeError(
                            l_location,
                            r_location,
                            format!("value '{}' must be an integer", v),
                        )))
                    }
                },
                Err(_) => {
                    return Err(EvaluationError::RuntimeError(RuntimeError::NullReference(
                        l_location, r_location,
                    )))
                }
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
                                    return Err(EvaluationError::RuntimeError(
                                        RuntimeError::TypeError(
                                            l_location,
                                            r_location,
                                            format!(
                                                "expression '{:?}' must be boolean",
                                                condition.unwrap()
                                            ),
                                        ),
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
                    return Err(EvaluationError::RuntimeError(RuntimeError::TypeError(
                        l_location,
                        r_location,
                        format!(
                            "Expression '{:?}' is not an Integer or Bool",
                            last_value_or_condition
                        ),
                    )))
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

    fn eval_one_line_statement(
        &mut self,
        statement: OneLineStatement,
    ) -> Result<(), EvaluationError> {
        let node = statement.node.clone();
        match node {
            OneLineStatementKind::Loop {
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
                            return Err(EvaluationError::RuntimeError(RuntimeError::TypeError(
                                iterator.l_location,
                                iterator.r_location,
                                format!(
                                    "Expression '{:?}' must be address (Integer) or Variable",
                                    &iterator
                                ),
                            )));
                        }
                    },
                    Err(e) => return Err(e),
                };

                let l_init_value = match self.eval_expression(initial_value.clone()) {
                    Ok(v) => match v {
                        Value::Int { value } => value,
                        _ => {
                            return Err(EvaluationError::RuntimeError(RuntimeError::TypeError(
                                initial_value.l_location,
                                initial_value.r_location,
                                format!("Expression '{:?}' must be Integer", &initial_value),
                            )));
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
                        _ => {
                            return Err(EvaluationError::RuntimeError(RuntimeError::TypeError(
                                step.l_location,
                                step.r_location,
                                format!("Expression '{:?}' must be Integer", &step),
                            )))
                        }
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
                                return Err(EvaluationError::RuntimeError(RuntimeError::TypeError(
                                    last_value_or_condition.l_location,
                                    last_value_or_condition.r_location,
                                    format!(
                                        "Expression '{:?}' must be Integer or Bool",
                                        &last_value_or_condition
                                    ),
                                )))
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
                    statement.l_location,
                    statement.r_location,
                )
            }
            OneLineStatementKind::Predicate {
                condition,
                if_true,
                if_false,
            } => {
                let cond = match self.eval_expression(condition.clone()) {
                    Ok(value) => match value {
                        Value::Bool { value } => value,
                        _ => {
                            return Err(EvaluationError::RuntimeError(RuntimeError::TypeError(
                                condition.l_location,
                                condition.r_location,
                                format!("Expression '{:?}' must be Integer or Bool", &condition),
                            )))
                        }
                    },
                    Err(e) => return Err(e),
                };
                if cond {
                    self.eval_statements(*if_true)
                } else {
                    self.eval_statements(*if_false)
                }
            }
            OneLineStatementKind::Exit => Ok(self.stop_eval()),
            OneLineStatementKind::UnconditionalJump { label } => {
                match self.env.get_line_by_label(&label) {
                    Some(line) => {
                        self.is_jumped = true;
                        self.current_line = line.clone();
                        Ok(())
                    }
                    None => {
                        return Err(EvaluationError::RuntimeError(RuntimeError::LabelNotFound(
                            statement.l_location,
                            statement.r_location,
                            label,
                        )))
                    }
                }
            }
            OneLineStatementKind::SubProgram { .. } => {
                return Err(EvaluationError::UnhandledFormula(
                    statement.l_location,
                    statement.r_location,
                    statement.node.clone(),
                ))
            }
            OneLineStatementKind::Return => {
                return Err(EvaluationError::UnhandledFormula(
                    statement.l_location,
                    statement.r_location,
                    statement.node.clone(),
                ))
            }
        }
    }

    fn eval_statement(&mut self, statement: SimpleStatement) -> Result<(), EvaluationError> {
        let node = &statement.node;
        match node {
            SimpleStatementKind::Expression { expression } => {
                if let Err(e) = self.eval_expression(expression.clone()) {
                    return Err(e);
                }

                Ok(())
            }

            SimpleStatementKind::Assign { lhs, rhs } => {
                let lhs_node = &lhs.node;
                let r = match self.eval_expression(rhs.clone()) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                };
                let l: Value = match lhs_node.clone() {
                    ExpressionKind::Null => todo!(),
                    ExpressionKind::Float { .. } => {
                        return Err(EvaluationError::RuntimeError(RuntimeError::TypeError(
                            lhs.l_location,
                            lhs.r_location,
                            format!("Expression '{:?}' must be address", &lhs),
                        )))
                    }
                    ExpressionKind::Bool { .. } => {
                        return Err(EvaluationError::RuntimeError(RuntimeError::TypeError(
                            lhs.l_location,
                            lhs.r_location,
                            format!("Expression '{:?}' must be address", &lhs),
                        )))
                    }
                    ExpressionKind::Int { .. } => self.eval_expression(lhs.clone())?,
                    ExpressionKind::String { .. } => {
                        return Err(EvaluationError::RuntimeError(RuntimeError::TypeError(
                            lhs.l_location,
                            lhs.r_location,
                            format!("Expression '{:?}' must be address", &lhs),
                        )))
                    }
                    ExpressionKind::Var { .. } => {
                        if let Value::Int { value } = r {
                            return match self.bind(&lhs, value) {
                                Ok(v) => Ok(v),
                                Err(_) => {
                                    return Err(EvaluationError::RuntimeError(
                                        RuntimeError::NullReference(lhs.l_location, lhs.r_location),
                                    ))
                                }
                            };
                        } else {
                            return Err(EvaluationError::RuntimeError(RuntimeError::TypeError(
                                lhs.l_location,
                                lhs.r_location,
                                format!(
                                    "Expression '{:?}' must be address",
                                    &lhs // TO CHECK
                                ),
                            )));
                        }
                    }
                    ExpressionKind::Call { .. } => self.eval_expression(lhs.clone())?,
                    ExpressionKind::UnaryOp { op, expr } => match op {
                        UnaryOp::Dereference => self.eval_expression(*expr)?,
                        UnaryOp::Not => self.eval_expression(lhs.clone())?,
                    },
                    ExpressionKind::BinaryOp { .. } => self.eval_expression(lhs.clone())?,
                };

                match l {
                    Value::Int { value } => Ok(self.env.fill_address(value, r)),
                    _ => {
                        return Err(EvaluationError::RuntimeError(RuntimeError::TypeError(
                            lhs.l_location,
                            lhs.r_location,
                            format!("Expression '{:?}' must be address", &lhs),
                        )))
                    }
                }
            }

            SimpleStatementKind::Send { lhs, rhs } => {
                let address = match self.eval_expression(lhs.clone()) {
                    Ok(Value::Int { value }) => value,
                    _ => {
                        return Err(EvaluationError::RuntimeError(RuntimeError::TypeError(
                            lhs.l_location,
                            lhs.r_location,
                            format!("Expression '{:?}' must be address", &lhs),
                        )))
                    }
                };

                let value = match self.eval_expression(rhs.clone()) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                };

                Ok(self.env.fill_address(address, value))
            }

            _ => Err(EvaluationError::UnhandledStatement(
                statement.l_location,
                statement.r_location,
                statement.node,
            )),
        }
    }

    fn bind(&mut self, lhs: &Expression, address: i64) -> Result<(), String> {
        let node = &lhs.node;
        match node {
            ExpressionKind::Var { name } => Ok(self.env.add_variable(&name, address)),
            _ => Err(format!("{:?} is not a variable", lhs)),
        }
    }

    fn eval_expression(&mut self, expression: Expression) -> Result<Value, EvaluationError> {
        let node = expression.node;
        match node {
            ExpressionKind::Int { value } => Ok(Value::Int { value }),
            ExpressionKind::Call { function, args } => {
                let mut vals = vec![];

                for arg in args {
                    match self.eval_expression(*arg) {
                        Ok(value) => vals.push(value),
                        Err(e) => return Err(e),
                    }
                }

                let v = match self.env.get_function(&function) {
                    Ok(v) => v,
                    Err(_) => {
                        return Err(EvaluationError::RuntimeError(
                            RuntimeError::FunctionNotFound(
                                expression.l_location,
                                expression.r_location,
                                function,
                            ),
                        ))
                    }
                };

                let func_name = function;

                if let Value::Function { function } = v {
                    match function(vals) {
                        Ok(v) => Ok(v),
                        Err(e) => {
                            return Err(EvaluationError::RuntimeError(
                                RuntimeError::FunctionCallError(
                                    expression.l_location,
                                    expression.r_location,
                                    func_name,
                                    e,
                                ),
                            ))
                        }
                    }
                } else {
                    Err(EvaluationError::RuntimeError(RuntimeError::TypeError(
                        expression.l_location,
                        expression.r_location,
                        format!("'{}' isn`t  function", func_name),
                    )))
                }
            }

            ExpressionKind::BinaryOp { op, lhs, rhs } => {
                let lv = match self.eval_expression(*lhs) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                };

                let rv = match self.eval_expression(*rhs) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                };

                match op {
                    BinaryOp::Sum => match Value::sum(lv, rv) {
                        Ok(v) => Ok(v),
                        Err(e) => Err(EvaluationError::RuntimeError(RuntimeError::TypeError(
                            expression.l_location,
                            expression.r_location,
                            e,
                        ))),
                    },
                    BinaryOp::Sub => match Value::sub(lv, rv) {
                        Ok(v) => Ok(v),
                        Err(e) => Err(EvaluationError::RuntimeError(RuntimeError::TypeError(
                            expression.l_location,
                            expression.r_location,
                            e,
                        ))),
                    },
                    BinaryOp::Mul => match Value::mul(lv, rv) {
                        Ok(v) => Ok(v),
                        Err(e) => Err(EvaluationError::RuntimeError(RuntimeError::TypeError(
                            expression.l_location,
                            expression.r_location,
                            e,
                        ))),
                    },
                    BinaryOp::EQ => Ok(Value::Bool { value: lv.eq(&rv) }),
                    BinaryOp::NE => Ok(Value::Bool { value: lv.ne(&rv) }),
                    BinaryOp::LT => Ok(Value::Bool { value: lv.lt(&rv) }),
                    _ => Err(EvaluationError::UnhandledBinaryOperation(
                        expression.l_location,
                        expression.r_location,
                        op,
                    )),
                }
            }

            ExpressionKind::Var { name } => match self.env.get_variable(&name) {
                Ok(v) => Ok(v),
                Err(_) => Err(EvaluationError::RuntimeError(
                    RuntimeError::VariableNotFound(
                        expression.l_location,
                        expression.r_location,
                        name,
                    ),
                )),
            },

            ExpressionKind::UnaryOp { op, expr } => match op {
                UnaryOp::Dereference => match self.eval_expression(*expr.clone()) {
                    Ok(Value::Int { value }) => match self.env.get_value_by_address(value) {
                        Ok(value) => Ok(value.clone()),
                        _ => Ok(Value::Null),
                    },
                    _ => {
                        return Err(EvaluationError::RuntimeError(RuntimeError::TypeError(
                            expr.l_location,
                            expr.r_location,
                            format!("Expression '{:?}' must be address", &expr),
                        )))
                    }
                },
                UnaryOp::Not => {
                    match self.eval_expression(*expr.clone()) {
                        Ok(value) => match value {
                            Value::Bool { value } => return Ok(Value::Bool { value: !value }),
                            _ => {
                                return Err(EvaluationError::RuntimeError(RuntimeError::TypeError(
                                    expr.l_location,
                                    expr.r_location,
                                    format!("Expression '{:?}' must be boolean", &expr),
                                )))
                            }
                        },
                        Err(e) => return Err(e),
                    };
                }
            },
            ExpressionKind::Bool { value } => Ok(Value::Bool { value }),
            ExpressionKind::String { value } => Ok(Value::String { value }),
            ExpressionKind::Float { value } => Ok(Value::Float { value }),
            ExpressionKind::Null => Ok(Value::Null),
        }
    }
}
