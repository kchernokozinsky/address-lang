use crate::*;
use self::typings::Type;
use super::{EvaluationError, RuntimeError};

impl Evaluator {

    fn process_lines_inside_loop(
        &mut self,
        line_from: usize,
        line_until: usize,
    ) -> Result<StatementResult, EvaluationError> {
        self.current_line = line_from;
        loop {
            let cur = self.current_line;
            let line: FileLine = self.lines[cur].clone();

            let statement_result = self.eval_file_line(line)?;

            match statement_result {
                StatementResult::Continue => {
                    self.current_line += 1;
                    if self.current_line > line_until {
                        return Ok(StatementResult::Continue);
                    }
                    if self.current_line >= self.lines.len() {
                        return Ok(StatementResult::FullStop);
                    }
                }
                StatementResult::FullStop => return Ok(StatementResult::FullStop),
                StatementResult::LocalStop => return Ok(StatementResult::LocalStop),
                StatementResult::JumpTo(line) => {
                    if line < line_from || line > line_until {
                        return Ok(StatementResult::JumpTo(line));
                    }
                    self.current_line = line
                }
            }
        }
    }

    pub fn eval_loop(
        &mut self,
        statement: OneLineStatement,
    ) -> Result<StatementResult, EvaluationError> {
        let l_location = statement.l_location;
        let r_location = statement.r_location;
        match &statement.node {
            OneLineStatementKind::Loop {
                initial_value,
                step,
                last_value_or_condition,
                iterator,
                label_until,
                label_to,
            } => {
                let initial = match self.eval_expression(initial_value.clone())?.extract_int() {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(EvaluationError::RuntimeError(
                            initial_value.l_location,
                            initial_value.r_location,
                            RuntimeError::TypeError(e),
                        ))
                    }
                };

                let step = match self.eval_expression(step.clone())?.extract_int() {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(EvaluationError::RuntimeError(
                            step.l_location,
                            step.r_location,
                            RuntimeError::TypeError(e),
                        ))
                    }
                };

                let iterator_v = match self.eval_expression(iterator.clone())?.extract_int() {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(EvaluationError::RuntimeError(
                            iterator.l_location,
                            iterator.r_location,
                            RuntimeError::TypeError(e),
                        ))
                    }
                };

                let last_value_or_condition_value =
                    self.eval_expression(last_value_or_condition.clone())?;
                let last_value_or_condition_type = Value::type_of(&last_value_or_condition_value);
                //
                // Evaluate lines number for evaluation inside loop
                //
                let temp = self.context.lookup_line_by_label(label_until);
                let line_until = match &temp {
                    Some(l) => **l,
                    None => {
                        return Err(EvaluationError::RuntimeError(
                            l_location,
                            r_location,
                            RuntimeError::LabelNotFound(label_until.to_string()),
                        ))
                    }
                };
                let line_from = self.current_line + 1;

                let line_to: usize =  match label_to {
                    Some(label_to) => {
                        match self.context.lookup_line_by_label(label_to) {
                            Some(l) => l.clone(),
                            None => {
                                return Err(EvaluationError::RuntimeError(
                                    l_location,
                                    r_location,
                                    RuntimeError::LabelNotFound(label_to.to_string()),
                                ))
                            }
                        }
                    },
                    None => line_until + 1,
                };
                
                //
                // Evaluate lines inside loop depending on condition
                //

                self.context.write_to_address(iterator_v, Value::new_int(initial));

                let mut iterator_value =
                    match self.context.read_from_address(iterator_v).extract_int() {
                        Ok(v) => v,
                        Err(e) => {
                            return Err(EvaluationError::RuntimeError(
                                iterator.l_location,
                                iterator.r_location,
                                RuntimeError::TypeError(e),
                            ))
                        }
                    };

                match &last_value_or_condition_type {
                    Type::Bool => {
                        let mut cond = match last_value_or_condition_value.extract_bool() {
                            Ok(v) => v,
                            Err(e) => {
                                return Err(EvaluationError::RuntimeError(
                                    last_value_or_condition.l_location,
                                    last_value_or_condition.r_location,
                                    RuntimeError::TypeError(e),
                                ))
                            }
                        };

                        loop {
                            if !cond {
                                return Ok(StatementResult::JumpTo(line_to));
                            }
                            match self
                                .process_lines_inside_loop(line_from.clone(), line_until.clone())?
                            {
                                StatementResult::Continue => {}
                                StatementResult::FullStop => return Ok(StatementResult::FullStop),
                                StatementResult::LocalStop => {
                                    return Ok(StatementResult::LocalStop)
                                }
                                StatementResult::JumpTo(l) => {
                                    return Ok(StatementResult::JumpTo(l))
                                }
                            };

                            iterator_value += step;
                            self.context
                                .write_to_address(iterator_v, Value::new_int(iterator_value));

                            cond = match self
                                .eval_expression(last_value_or_condition.clone())?
                                .extract_bool()
                            {
                                Ok(v) => v,
                                Err(e) => {
                                    return Err(EvaluationError::RuntimeError(
                                        last_value_or_condition.l_location,
                                        last_value_or_condition.r_location,
                                        RuntimeError::TypeError(e),
                                    ))
                                }
                            };
                        }
                    }
                    Type::Int => {

                        let last_value = match last_value_or_condition_value.extract_int() {
                            Ok(v) => v,
                            Err(e) => {
                                return Err(EvaluationError::RuntimeError(
                                    last_value_or_condition.l_location,
                                    last_value_or_condition.r_location,
                                    RuntimeError::TypeError(e),
                                ))
                            }
                        };
                        let mut cond = iterator_value < last_value;

                        loop {
                            if !cond {
                                return Ok(StatementResult::JumpTo(line_to));
                            }
                            match self
                                .process_lines_inside_loop(line_from.clone(), line_until.clone())?
                            {
                                StatementResult::Continue => {}
                                StatementResult::FullStop => return Ok(StatementResult::FullStop),
                                StatementResult::LocalStop => {
                                    return Ok(StatementResult::LocalStop)
                                }
                                StatementResult::JumpTo(l) => {
                                    return Ok(StatementResult::JumpTo(l))
                                }
                            };

                            iterator_value += step;
                            
                            self.context
                                .write_to_address(iterator_v, Value::new_int(iterator_value));

                            cond = iterator_value < last_value;
                        }
                    }
                    _ => {
                        return Err(EvaluationError::RuntimeError(
                            last_value_or_condition.l_location,
                            last_value_or_condition.r_location,
                            RuntimeError::TypeError(Value::_raise_unexpected_type_error(
                                vec![Type::Int, Type::Bool],
                                &last_value_or_condition_value,
                            )),
                        ))
                    }
                }
            }
            _ => return Ok(StatementResult::Continue),
        };
    }
    
}
