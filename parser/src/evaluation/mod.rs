pub mod builtins;
pub mod errors;
pub mod runtime_context;
pub mod value;

use crate::ast::*;
use crate::location::Location;
use crate::typings::Type;
use errors::*;
use runtime_context::*;
use value::*;

pub struct Evaluator {
    lines: Vec<FileLine>,
    context: RuntimeContext,
    current_line: usize,
    to_stop: bool,
    is_jumped: bool,
}

impl Evaluator {
    pub fn new(lines: Vec<FileLine>, env: RuntimeContext) -> Evaluator {
        let mut evaluator = Evaluator {
            lines,
            context: env,
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
                self.context.register_label(label.to_string(), index);
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
            .context
            .lookup_line_by_label(label_until)
            .expect(format!("label '{}' is not declared!", &label_until).as_str());

        let line_to_jump = match self.context.lookup_line_by_label(label_to) {
            Some(line) => *line,
            None => {
                return Err(EvaluationError::RuntimeError(
                    l_location,
                    r_location,
                    RuntimeError::LabelNotFound(label_to.to_string()),
                ));
            }
        };
        let mut it_just_set: bool = true;
        self.current_line = line_from;
        loop {
            let mut it_value = match self.context.read_from_address(iterator) {
                Value::Int(value) => *value,
                v => {
                    return Err(EvaluationError::RuntimeError(
                        l_location,
                        r_location,
                        RuntimeError::TypeError(Value::raise_unexpected_type_error(Type::Int, &v)),
                    ))
                }
            };
            // increment iterator
            if !it_just_set {
                it_value = it_value + step;
                self.context
                    .write_to_address(iterator, Value::new_int(it_value));
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
                                Value::Bool(value) => value,
                                _ => {
                                    return Err(EvaluationError::RuntimeError(
                                        l_location,
                                        r_location,
                                        RuntimeError::TypeError(
                                            Value::raise_unexpected_type_error(Type::Bool, &v),
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

                Value::Int(value) => {
                    if it_value <= value {
                    } else {
                        self.is_jumped = true;
                        self.current_line = line_to_jump;
                        return Ok(());
                    }
                }

                _ => {
                    return Err(EvaluationError::RuntimeError(
                        l_location,
                        r_location,
                        RuntimeError::TypeError(Value::_raise_unexpected_type_error(
                            vec![Type::Int, Type::Bool],
                            &last_value_or_condition,
                        )),
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
                    Ok(v) => match v.extract_int() {
                        Ok(v) => v,
                        Err(e) => {
                            return Err(EvaluationError::RuntimeError(
                                iterator.l_location,
                                iterator.r_location,
                                RuntimeError::TypeError(e),
                            ))
                        }
                    },
                    Err(e) => return Err(e),
                };

                let l_init_value = match self.eval_expression(initial_value.clone()) {
                    Ok(v) => match v.extract_int() {
                        Ok(v) => v,
                        Err(e) => {
                            return Err(EvaluationError::RuntimeError(
                                initial_value.l_location,
                                initial_value.r_location,
                                RuntimeError::TypeError(e),
                            ))
                        }
                    },
                    Err(e) => return Err(e),
                };

                self.context
                    .write_to_address(l_it_adrress, Value::new_int(l_init_value));

                let l_step = match self.eval_expression(step.clone()) {
                    Ok(v) => match v.extract_int() {
                        Ok(v) => v,
                        Err(e) => {
                            return Err(EvaluationError::RuntimeError(
                                step.l_location,
                                step.r_location,
                                RuntimeError::TypeError(e),
                            ))
                        }
                    },
                    Err(e) => return Err(e),
                };

                let mut cond = None;
                let l_last_value_or_condition: Value =
                    match self.eval_expression(last_value_or_condition.clone()) {
                        Ok(v) => match v {
                            Value::Int(value) => Value::Int(value),
                            Value::Bool(value) => {
                                cond = Some(last_value_or_condition.clone());
                                Value::Bool(value)
                            }
                            v => {
                                return Err(EvaluationError::RuntimeError(
                                    last_value_or_condition.l_location,
                                    last_value_or_condition.r_location,
                                    RuntimeError::TypeError(Value::_raise_unexpected_type_error(
                                        vec![Type::Int, Type::Bool],
                                        &v,
                                    )),
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
                        Value::Bool(value) => value,
                        v => {
                            return Err(EvaluationError::RuntimeError(
                                condition.l_location,
                                condition.r_location,
                                RuntimeError::TypeError(Value::_raise_unexpected_type_error(
                                    vec![Type::Int, Type::Bool],
                                    &v,
                                )),
                            ))
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
                match self.context.lookup_line_by_label(&label) {
                    Some(line) => {
                        self.is_jumped = true;
                        self.current_line = line.clone();
                        Ok(())
                    }
                    None => {
                        return Err(EvaluationError::RuntimeError(
                            statement.l_location,
                            statement.r_location,
                            RuntimeError::LabelNotFound(label),
                        ))
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

    fn assign_to_dereference(
        &mut self,
        lhs: &Located<ExpressionKind>,
        rhs: &Located<ExpressionKind>,
    ) -> Result<(), EvaluationError> {
        let address = match self.eval_expression(lhs.clone())?.extract_int() {
            Ok(address) => address,
            Err(e) => {
                return Err(EvaluationError::RuntimeError(
                    lhs.l_location,
                    lhs.r_location,
                    RuntimeError::TypeError(e),
                ))
            }
        };

        let value = self.eval_expression(rhs.clone())?;

        Ok(self.context.write_to_address(address, value))
    }

    fn assign_to_variable(
        &mut self,
        variable: &String,
        rhs: &Located<ExpressionKind>,
    ) -> Result<(), EvaluationError> {
        let address = match self.eval_expression(rhs.clone())?.extract_int() {
            Ok(address) => address,
            Err(e) => {
                return Err(EvaluationError::RuntimeError(
                    rhs.l_location,
                    rhs.r_location,
                    RuntimeError::TypeError(e),
                ))
            }
        };

        Ok(self.context.add_variable(variable, address))
    }

    fn assign_to_address(
        &mut self,
        address: i64,
        rhs: &Located<ExpressionKind>,
    ) -> Result<(), EvaluationError> {
        let value = self.eval_expression(rhs.clone())?;
        Ok(self.context.write_to_address(address, value))
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
                let lhs_node: &ExpressionKind = &lhs.node;

                //
                //decide which strategy for evaluation, to choose
                //
                match lhs_node {
                    ExpressionKind::Null => {
                        return Err(EvaluationError::RuntimeError(
                            lhs.l_location,
                            lhs.r_location,
                            RuntimeError::TypeError(ValueError::UnexpectedType {
                                expected_type: Type::Int,
                                actual_type: Type::Null,
                                actual_value: format!("{}", Value::Null),
                            }),
                        ))
                    }
                    ExpressionKind::Float { value } => {
                        return Err(EvaluationError::RuntimeError(
                            lhs.l_location,
                            lhs.r_location,
                            RuntimeError::TypeError(ValueError::UnexpectedType {
                                expected_type: Type::Int,
                                actual_type: Type::Float,
                                actual_value: format!("{}", value),
                            }),
                        ))
                    }
                    ExpressionKind::Bool { value } => {
                        return Err(EvaluationError::RuntimeError(
                            lhs.l_location,
                            lhs.r_location,
                            RuntimeError::TypeError(ValueError::UnexpectedType {
                                expected_type: Type::Int,
                                actual_type: Type::Bool,
                                actual_value: format!("{}", value),
                            }),
                        ))
                    }
                    ExpressionKind::Int { value } => return self.assign_to_address(*value, rhs),
                    ExpressionKind::String { value } => {
                        return Err(EvaluationError::RuntimeError(
                            lhs.l_location,
                            lhs.r_location,
                            RuntimeError::TypeError(ValueError::UnexpectedType {
                                expected_type: Type::Int,
                                actual_type: Type::String,
                                actual_value: format!("{}", value),
                            }),
                        ))
                    }
                    ExpressionKind::Var { name } => return self.assign_to_variable(name, rhs),
                    ExpressionKind::Call { function, .. } => {
                        return Err(EvaluationError::RuntimeError(
                            lhs.l_location,
                            lhs.r_location,
                            RuntimeError::TypeError(ValueError::UnexpectedType {
                                expected_type: Type::Int,
                                actual_type: Type::Function,
                                actual_value: format!("{}", function),
                            }),
                        ))
                    }
                    ExpressionKind::UnaryOp { op, expr } => match op {
                        UnaryOp::Dereference => self.assign_to_dereference(expr, rhs),
                        UnaryOp::Not => match self.eval_expression(lhs.clone())?.extract_int() {
                            Ok(address) => self.assign_to_address(address, rhs),
                            Err(e) => {
                                return Err(EvaluationError::RuntimeError(
                                    lhs.l_location,
                                    lhs.r_location,
                                    RuntimeError::TypeError(e),
                                ))
                            }
                        },
                        UnaryOp::MultipleDereference(expression) => {
                            match self.eval_expression(*expression.clone()) {
                                Ok(value) => match value.extract_int() {
                                    Ok(n) => {
                                        match self
                                            .derefence_n_times(
                                                *expr.clone(),
                                                n - 1,
                                                lhs.l_location,
                                                lhs.r_location,
                                            )?
                                            .extract_int()
                                        {
                                            Ok(address) => {
                                                return self.assign_to_address(address, rhs)
                                            }
                                            Err(e) => {
                                                return Err(EvaluationError::RuntimeError(
                                                    lhs.l_location,
                                                    lhs.r_location,
                                                    RuntimeError::TypeError(e),
                                                ))
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        return Err(EvaluationError::RuntimeError(
                                            expr.l_location,
                                            expr.r_location,
                                            RuntimeError::TypeError(e),
                                        ))
                                    }
                                },
                                Err(e) => return Err(e),
                            };
                        }
                    },
                    ExpressionKind::BinaryOp { .. } => {
                        match self.eval_expression(lhs.clone())?.extract_int() {
                            Ok(address) => self.assign_to_address(address, rhs),
                            Err(e) => {
                                return Err(EvaluationError::RuntimeError(
                                    lhs.l_location,
                                    lhs.r_location,
                                    RuntimeError::TypeError(e),
                                ))
                            }
                        }
                    }
                    ExpressionKind::List { .. } => {
                        match self.eval_expression(lhs.clone())?.extract_int() {
                            Ok(address) => return self.assign_to_address(address, rhs),
                            Err(e) => {
                                return Err(EvaluationError::RuntimeError(
                                    lhs.l_location,
                                    lhs.r_location,
                                    RuntimeError::TypeError(e),
                                ))
                            }
                        };
                    }
                }
            }

            SimpleStatementKind::Send { lhs, rhs } => {
                let address = match self.eval_expression(lhs.clone()) {
                    Ok(v) => match v.extract_int() {
                        Ok(v) => v,
                        Err(e) => {
                            return Err(EvaluationError::RuntimeError(
                                lhs.l_location,
                                lhs.r_location,
                                RuntimeError::TypeError(e),
                            ))
                        }
                    },
                    Err(e) => return Err(e),
                };

                let value = match self.eval_expression(rhs.clone()) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                };

                Ok(self.context.write_to_address(address, value))
            }

            _ => Err(EvaluationError::UnhandledStatement(
                statement.l_location,
                statement.r_location,
                statement.node,
            )),
        }
    }

    fn eval_expression(&mut self, expression: Expression) -> Result<Value, EvaluationError> {
        let node = expression.node;
        match node {
            ExpressionKind::Int { value } => Ok(Value::Int(value)),
            ExpressionKind::Call { function, args } => {
                let mut vals = vec![];

                for arg in args {
                    match self.eval_expression(*arg) {
                        Ok(value) => vals.push(value),
                        Err(e) => return Err(e),
                    }
                }

                let v = match self.context.get_function(&function) {
                    Ok(v) => v,
                    Err(_) => {
                        return Err(EvaluationError::RuntimeError(
                            expression.l_location,
                            expression.r_location,
                            RuntimeError::FunctionNotFound(function),
                        ))
                    }
                };

                let func_name = function;

                if let Value::Function(function) = v {
                    match function(vals) {
                        Ok(v) => Ok(v),
                        Err(e) => {
                            return Err(EvaluationError::RuntimeError(
                                expression.l_location,
                                expression.r_location,
                                RuntimeError::FunctionCallError(func_name, e),
                            ))
                        }
                    }
                } else {
                    Err(EvaluationError::RuntimeError(
                        expression.l_location,
                        expression.r_location,
                        RuntimeError::TypeError(ValueError::UnexpectedType {
                            expected_type: Type::Function,
                            actual_type: Value::type_of(v),
                            actual_value: format!("{}", v),
                        }),
                    ))
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
                    BinaryOp::Sum => match Value::sum(&lv, &rv) {
                        Ok(v) => Ok(v),
                        Err(e) => Err(EvaluationError::RuntimeError(
                            expression.l_location,
                            expression.r_location,
                            RuntimeError::TypeError(e),
                        )),
                    },
                    BinaryOp::Sub => match Value::sub(&lv, &rv) {
                        Ok(v) => Ok(v),
                        Err(e) => Err(EvaluationError::RuntimeError(
                            expression.l_location,
                            expression.r_location,
                            RuntimeError::TypeError(e),
                        )),
                    },
                    BinaryOp::Mul => match Value::mul(&lv, &rv) {
                        Ok(v) => Ok(v),
                        Err(e) => Err(EvaluationError::RuntimeError(
                            expression.l_location,
                            expression.r_location,
                            RuntimeError::TypeError(e),
                        )),
                    },
                    BinaryOp::EQ => match Value::eq(&lv, &rv) {
                        Ok(v) => Ok(v),
                        Err(e) => Err(EvaluationError::RuntimeError(
                            expression.l_location,
                            expression.r_location,
                            RuntimeError::TypeError(e),
                        )),
                    },
                    BinaryOp::NE => match Value::ne(&lv, &rv) {
                        Ok(v) => Ok(v),
                        Err(e) => Err(EvaluationError::RuntimeError(
                            expression.l_location,
                            expression.r_location,
                            RuntimeError::TypeError(e),
                        )),
                    },
                    BinaryOp::LT => match Value::lt(&lv, &rv) {
                        Ok(v) => Ok(v),
                        Err(e) => Err(EvaluationError::RuntimeError(
                            expression.l_location,
                            expression.r_location,
                            RuntimeError::TypeError(e),
                        )),
                    },
                    BinaryOp::GT => match Value::gt(&lv, &rv) {
                        Ok(v) => Ok(v),
                        Err(e) => Err(EvaluationError::RuntimeError(
                            expression.l_location,
                            expression.r_location,
                            RuntimeError::TypeError(e),
                        )),
                    },
                    BinaryOp::Div => match Value::div(&lv, &rv) {
                        Ok(v) => Ok(v),
                        Err(e) => Err(EvaluationError::RuntimeError(
                            expression.l_location,
                            expression.r_location,
                            RuntimeError::TypeError(e),
                        )),
                    },
                    BinaryOp::Mod => todo!(),
                    BinaryOp::And => match Value::and(lv, rv) {
                        Ok(v) => Ok(v),
                        Err(e) => Err(EvaluationError::RuntimeError(
                            expression.l_location,
                            expression.r_location,
                            RuntimeError::TypeError(e),
                        )),
                    },
                    BinaryOp::Or => match Value::or(lv, rv) {
                        Ok(v) => Ok(v),
                        Err(e) => Err(EvaluationError::RuntimeError(
                            expression.l_location,
                            expression.r_location,
                            RuntimeError::TypeError(e),
                        )),
                    },
                }
            }

            ExpressionKind::Var { name } => match self.context.get_variable_address(&name) {
                Ok(v) => Ok(Value::new_int(v)),
                Err(_) => Ok(Value::new_int(self.context.allocate_variable(&name)))
                // Err(EvaluationError::RuntimeError(
                //     expression.l_location,
                //     expression.r_location,
                //     RuntimeError::VariableNotFound(name),
                // )),
            },

            ExpressionKind::UnaryOp { op, expr } => match op {
                UnaryOp::Dereference => match self.eval_expression(*expr.clone()) {
                    Ok(v) => {
                        match v.extract_int() {
                            Ok(v) => return Ok(self.context.read_from_address(v).clone()),
                            Err(e) => {
                                return Err(EvaluationError::RuntimeError(
                                    expr.l_location,
                                    expr.r_location,
                                    RuntimeError::TypeError(e),
                                ))
                            }
                        };
                    }
                    Err(e) => return Err(e),
                },
                UnaryOp::Not => {
                    match self.eval_expression(*expr.clone()) {
                        Ok(value) => match value.extract_bool() {
                            Ok(v) => return Ok(Value::new_bool(!v)),
                            Err(e) => {
                                return Err(EvaluationError::RuntimeError(
                                    expr.l_location,
                                    expr.r_location,
                                    RuntimeError::TypeError(e),
                                ))
                            }
                        },
                        Err(e) => return Err(e),
                    };
                }
                UnaryOp::MultipleDereference(expression) => {
                    match self.eval_expression(*expression.clone()) {
                        Ok(value) => match value.extract_int() {
                            Ok(n) => {
                                return self.derefence_n_times(
                                    *expr,
                                    n,
                                    expression.l_location,
                                    expression.r_location,
                                )
                            }
                            Err(e) => {
                                return Err(EvaluationError::RuntimeError(
                                    expr.l_location,
                                    expr.r_location,
                                    RuntimeError::TypeError(e),
                                ))
                            }
                        },
                        Err(e) => return Err(e),
                    };
                }
            },
            ExpressionKind::Bool { value } => Ok(Value::new_bool(value)),
            ExpressionKind::String { value } => Ok(Value::new_string(value)),
            ExpressionKind::Float { value } => Ok(Value::new_float(value)),
            ExpressionKind::Null => Ok(Value::Null),
            ExpressionKind::List { elements } => {
                let mut v_elements = vec![];
                for e in elements {
                    v_elements.push(self.eval_expression(*e)?)
                }
                Ok(Value::new_int(self.context.allocate_list(v_elements)))
            }
        }
    }

    fn derefence_n_times(
        &mut self,
        expression: Expression,
        n: i64,
        l_location: Location,
        r_location: Location,
    ) -> Result<Value, EvaluationError> {
        let val = self.eval_expression(expression.clone())?;
        if n == 0 {
            return Ok(val);
        };
        let mut address = match val.extract_int() {
            Ok(v) => v,
            Err(e) => {
                return Err(EvaluationError::RuntimeError(
                    expression.l_location,
                    expression.r_location,
                    RuntimeError::TypeError(e),
                ))
            }
        };

        let mut i = 1;
        loop {
            let temp = self.context.read_from_address(address);
            if i >= n {
                return Ok(temp.clone());
            }

            address = match temp.extract_int() {
                Ok(v) => v,
                Err(e) => {
                    return Err(EvaluationError::RuntimeError(
                        l_location,
                        r_location,
                        RuntimeError::TypeError(e),
                    ))
                }
            };
            i+=1;
        }
    }
}
