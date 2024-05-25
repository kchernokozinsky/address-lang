pub mod builtins;
pub mod errors;
pub mod loop_;
pub mod runtime_context;
pub mod subprogram;

use common::location::Location;
use errors::*;
use parser::ast::*;
use runtime_context::*;
use value::error::ValueError;
use value::typings::Type;
use value::*;
pub struct Evaluator {
    lines: Vec<FileLine>,
    context: RuntimeContext,
    current_line: usize,
}

pub enum StatementResult {
    Continue,
    FullStop,
    LocalStop,
    JumpTo(usize),
}

impl Evaluator {
    pub fn new(lines: Vec<FileLine>, context: RuntimeContext) -> Evaluator {
        let evaluator = Evaluator {
            lines,
            context,
            current_line: 0,
        };
        evaluator
    }

    pub fn increment_line(&mut self) {
        self.current_line += 1;
    }

    fn extract_labels(&mut self) -> Result<(), EvaluationError> {
        for (index, line) in self.lines.iter().enumerate() {
            let labels = line.labels();
            for label in labels {
                match self.context.register_label(label.to_string(), index) {
                    Ok(_) => {}
                    Err(e) => return Err(EvaluationError::RuntimeErrorWithoutLocation(e)),
                };
            }
        }
        Ok(())
    }

    pub fn eval(&mut self) -> Result<(), EvaluationError> {
        self.extract_labels()?;

        loop {
            let cur = self.current_line;
            let line: FileLine = self.lines[cur].clone();

            let statement_result = self.eval_file_line(line)?;

            match statement_result {
                StatementResult::Continue => {
                    self.current_line += 1;
                    if self.current_line >= self.lines.len() {
                        break;
                    }
                }
                StatementResult::FullStop => return Ok(()),
                StatementResult::LocalStop => return Ok(()),
                StatementResult::JumpTo(line) => self.current_line = line,
            }
        }
        Ok(())
    }

    fn eval_file_line(&mut self, line: FileLine) -> Result<StatementResult, EvaluationError> {
        match line {
            FileLine::Line {
                labels: _s,
                statements,
            } => self.eval_statements(statements),
        }
    }

    pub fn eval_statements(
        &mut self,
        statements: Statements,
    ) -> Result<StatementResult, EvaluationError> {
        match statements {
            Statements::OneLineStatement(stmnt) => self.eval_one_line_statement(stmnt),
            Statements::SimpleStatements(stmnts) => {
                for statement in stmnts {
                    let statement_result = self.eval_statement(statement)?;
                    match statement_result {
                        StatementResult::Continue => (),
                        StatementResult::FullStop => return Ok(StatementResult::FullStop),
                        StatementResult::LocalStop => return Ok(StatementResult::LocalStop),
                        StatementResult::JumpTo(line) => return Ok(StatementResult::JumpTo(line)),
                    }
                }
                Ok(StatementResult::Continue)
            }
        }
    }

    fn eval_one_line_statement(
        &mut self,
        statement: OneLineStatement,
    ) -> Result<StatementResult, EvaluationError> {
        let node = statement.node.clone();
        match node {
            OneLineStatementKind::Loop { .. } => self.eval_loop(statement),
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
            OneLineStatementKind::Exit => Ok(StatementResult::FullStop),
            OneLineStatementKind::UnconditionalJump { label } => {
                match self.context.lookup_line_by_label(&label) {
                    Some(line) => Ok(StatementResult::JumpTo(*line)),
                    None => {
                        return Err(EvaluationError::RuntimeError(
                            statement.l_location,
                            statement.r_location,
                            RuntimeError::LabelNotFound(label),
                        ))
                    }
                }
            }
            OneLineStatementKind::SubProgram { .. } => return self.eval_subprogram_call(statement),
            OneLineStatementKind::Return => return Ok(StatementResult::LocalStop),
        }
    }

    fn assign_to_dereference(
        &mut self,
        lhs: &Located<ExpressionKind>,
        rhs: &Located<ExpressionKind>,
    ) -> Result<StatementResult, EvaluationError> {
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

        self.context.write_to_address(address, value);
        Ok(StatementResult::Continue)
    }

    fn assign_to_variable(
        &mut self,
        variable: &String,
        rhs: &Located<ExpressionKind>,
    ) -> Result<StatementResult, EvaluationError> {
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
        self.context.add_variable(variable, address);
        Ok(StatementResult::Continue)
    }

    fn assign_to_address(
        &mut self,
        address: i64,
        rhs: &Located<ExpressionKind>,
    ) -> Result<StatementResult, EvaluationError> {
        let value = self.eval_expression(rhs.clone())?;
        self.context.write_to_address(address, value);
        Ok(StatementResult::Continue)
    }

    fn eval_statement(
        &mut self,
        statement: SimpleStatement,
    ) -> Result<StatementResult, EvaluationError> {
        let node = &statement.node;
        match node {
            SimpleStatementKind::Expression { expression } => {
                if let Err(e) = self.eval_expression(expression.clone()) {
                    return Err(e);
                }

                Ok(StatementResult::Continue)
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
                        UnaryOp::Minus => todo!(),
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
                self.context.write_to_address(address, value);
                Ok(StatementResult::Continue)
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
                Err(_) => Ok(Value::new_int(self.context.allocate_variable(&name))),
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
                UnaryOp::Minus => todo!(),
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
            i += 1;
        }
    }
}
