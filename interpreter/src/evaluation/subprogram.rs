use super::*;

impl Evaluator {
    pub fn eval_subprogram_call(
        &mut self,
        statement: OneLineStatement,
    ) -> Result<StatementResult, EvaluationError> {
        let l_location = statement.l_location;
        let r_location = statement.r_location;
        match &statement.node {
            OneLineStatementKind::SubProgram {
                sp_name,
                args,
                label_to,
            } => {
                let sp_line = match self.context.lookup_line_by_label(&sp_name.identifier) {
                    Some(l) => l.clone(),
                    None => {
                        return Err(EvaluationError::RuntimeError(
                            l_location,
                            r_location,
                            RuntimeError::LabelNotFound(sp_name.to_string()),
                        ))
                    }
                };

                let line_to = match label_to {
                    Some(label_to) => match self.context.lookup_line_by_label(label_to) {
                        Some(l) => l.clone(),
                        None => {
                            return Err(EvaluationError::RuntimeError(
                                l_location,
                                r_location,
                                RuntimeError::LabelNotFound(label_to.to_string()),
                            ))
                        }
                    },
                    None => self.current_line + 1,
                };

                let agrs_len = args.len();

                self.current_line = sp_line;
                let cur = self.current_line;
                let line: FileLine = self.lines[cur].clone();

                match line {
                    FileLine::Line {
                        labels: _,
                        statements,
                    } => match statements {
                        Statements::OneLineStatement(one_line_statement) => {
                            return Err(EvaluationError::SubProgramDeclaration(
                                one_line_statement.l_location,
                                one_line_statement.r_location,
                                sp_name.to_string(),
                            ))
                        }
                        Statements::SimpleStatements(statements) => {
                            let mut vars: Vec<String> = vec![];

                            //
                            // Collect variables name
                            //
                            for statement in statements.clone() {
                                match statement.node {
                                    SimpleStatementKind::Assign { .. } => todo!(), // raise error,
                                    SimpleStatementKind::Send { lhs, rhs } => {
                                        match rhs.node {
                                            ExpressionKind::Null => (),
                                            _ => todo!(), // raise error,
                                        }

                                        match lhs.node {
                                            ExpressionKind::Var { name } => vars.push(name),
                                            _ => todo!(),
                                        }
                                    }
                                    SimpleStatementKind::Exchange { .. } => todo!(), // raise error,
                                    SimpleStatementKind::Expression { .. } => todo!(),
                                    SimpleStatementKind::Import {
                                        labels,
                                        path,
                                        alias,
                                    } => todo!(),
                                    SimpleStatementKind::Del { rhs } => todo!(),
                                }
                            }
                            //
                            // Check arguments number
                            //
                            if statements.len() != agrs_len {
                                return Err(EvaluationError::SubProgram(
                                    args[0].l_location,
                                    args[agrs_len - 1].r_location,
                                    RuntimeError::InvalidArgumentsNumber(
                                        sp_name.to_string(),
                                        statements.len(),
                                        agrs_len,
                                    ),
                                ));
                            };

                            //
                            // Evaluate parameters expressions
                            //

                            let mut addresses: Vec<i64> = vec![];
                            for statement in args.clone() {
                                match self.eval_expression(*statement.clone())?.extract_int() {
                                    Ok(e) => addresses.push(e),
                                    Err(e) => {
                                        return Err(EvaluationError::RuntimeError(
                                            statement.l_location,
                                            statement.r_location,
                                            RuntimeError::TypeError(e),
                                        ))
                                    }
                                };
                            }

                            let zipped: Vec<(i64, String)> = addresses
                                .into_iter()
                                .zip(vars.clone().into_iter())
                                .collect();
                            for (address, var) in zipped {
                                self.context.add_variable(&var, address);
                            }

                            self.current_line += 1;

                            loop {
                                let cur = self.current_line;
                                let line: FileLine = self.lines[cur].clone();

                                let statement_result = self.eval_file_line(line)?;

                                match statement_result {
                                    StatementResult::Continue => {
                                        self.current_line += 1;
                                        if self.current_line >= self.lines.len() {
                                            return Ok(StatementResult::FullStop);
                                        }
                                    }
                                    StatementResult::FullStop => {
                                        return Ok(StatementResult::FullStop)
                                    }
                                    StatementResult::LocalStop => {
                                        for name in vars.iter() {
                                            self.context.free_variable(name)
                                        }
                                        return Ok(StatementResult::JumpTo(line_to));
                                    }
                                    StatementResult::JumpTo(line) => self.current_line = line,
                                }
                            }
                        }
                    },
                }
            }
            _ => return Ok(StatementResult::Continue),
        }
    }
}
