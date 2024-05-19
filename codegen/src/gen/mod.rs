use std::{collections::HashMap, ops::Deref};

use parser::ast::{visitor::Visitor, *};
use value::Value;

use crate::bytecode::Bytecode;
mod tests;
pub struct BytecodeGenerator<'a> {
    bytecode: Vec<Bytecode>,
    labels: HashMap<String, usize>,
    jumps: Vec<(usize, String)>,
    loop_context: Vec<LoopContext>,
    ast: &'a Algorithm,      // Reference to the AST
    current_position: usize, // Track the current position in the AST
}

struct LoopContext {
    start_label: String,
    end_label: String,
}

impl<'a> BytecodeGenerator<'a> {
    pub fn new(ast: &'a Algorithm) -> Self {
        Self {
            bytecode: Vec::new(),
            labels: HashMap::new(),
            jumps: Vec::new(),
            loop_context: Vec::new(),
            ast,
            current_position: 0,
        }
    }

    pub fn get_bytecode(mut self) -> Vec<Bytecode> {
        self.resolve_jumps();
        self.bytecode
    }

    fn resolve_jumps(&mut self) {
        for (pos, label) in &self.jumps {
            if let Some(&address) = self.labels.get(label) {
                self.bytecode[*pos] = match self.bytecode[*pos] {
                    Bytecode::Jump(_) => Bytecode::Jump(address),
                    Bytecode::JumpIfFalse(_) => Bytecode::JumpIfFalse(address),
                    _ => todo!(),
                }
            } else {
                panic!("Undefined label: {}", label);
            }
        }
    }

    fn visit_loop_body(&mut self, label_until: &str) {
        if let Algorithm::Body(lines) = self.ast {
            while self.current_position < lines.len() {
                let line = &lines[self.current_position];
                if let FileLine::Line { labels, statements } = line {
                    if labels.contains(&label_until.to_string()) {
                        break;
                    }
                    statements.accept(self);
                }
                self.current_position += 1;
            }
        }
    }
}

impl<'a> Visitor for BytecodeGenerator<'a> {
    fn visit_algorithm(&mut self, algorithm: &Algorithm) {
        if let Algorithm::Body(lines) = algorithm {
            self.current_position = 0;
            while self.current_position < lines.len() {
                lines[self.current_position].accept(self);
                self.current_position += 1;
            }
        }
    }

    fn visit_file_line(&mut self, file_line: &FileLine) {
        if let FileLine::Line { labels, statements } = file_line {
            for label in labels {
                self.labels.insert(label.clone(), self.bytecode.len());
                self.bytecode.push(Bytecode::Label(label.clone()));
            }
            statements.accept(self);
        }
    }

    fn visit_statements(&mut self, statements: &Statements) {
        match statements {
            Statements::OneLineStatement(statement) => statement.accept(self),
            Statements::SimpleStatements(stmts) => {
                for stmt in stmts {
                    stmt.accept(self);
                }
            }
        }
    }

    fn visit_one_line_statement(&mut self, statement: &OneLineStatement) {
        statement.node.accept(self);
    }

    fn visit_one_line_statement_kind(&mut self, kind: &OneLineStatementKind) {
        match kind {
            OneLineStatementKind::Loop {
                initial_value,
                step,
                last_value_or_condition,
                iterator,
                label_until,
                label_to,
            } => {
                // Evaluate the initial value
                initial_value.accept(self);

                // Link the iterator variable to an address
                if let ExpressionKind::Var { name } = &iterator.node {
                    self.bytecode.push(Bytecode::SetVar(name.clone()));
                    self.bytecode.push(Bytecode::GetVar(name.clone()));
                    self.bytecode.push(Bytecode::Alloc);
                    // Label to mark the start of the loop
                    let start_label = format!("loop_start_{}", self.bytecode.len());
                    self.labels.insert(start_label.clone(), self.bytecode.len());
                    self.bytecode.push(Bytecode::Label(start_label.clone()));

                    // Evaluate loop condition
                    last_value_or_condition.accept(self);
                    let jump_if_false_pos = self.bytecode.len();
                    self.bytecode.push(Bytecode::JumpIfFalse(0)); // Placeholder

                    // Save the current loop context
                    let end_label = format!("loop_end_{}", self.bytecode.len());
                    self.loop_context.push(LoopContext {
                        start_label: start_label.clone(),
                        end_label: end_label.clone(),
                    });

                    // Generate bytecode for the loop body
                    let body_start_label = format!("loop_body_start_{}", self.bytecode.len());
                    self.labels
                        .insert(body_start_label.clone(), self.bytecode.len());
                    self.bytecode
                        .push(Bytecode::Label(body_start_label.clone()));

                    // Visit the loop body lines
                    self.current_position += 1; // Move to the first line of the loop body
                    self.visit_loop_body(label_until);

                    // Increment the loop variable
                    step.accept(self);
                    self.bytecode.push(Bytecode::GetVar(name.clone()));
                    self.bytecode.push(Bytecode::Deref);
                    self.bytecode.push(Bytecode::Add);
                    self.bytecode.push(Bytecode::GetVar(name.clone()));
                    self.bytecode.push(Bytecode::Alloc);
                    // Jump back to the start of the loop
                    self.bytecode
                        .push(Bytecode::Jump(self.labels[&start_label]));

                    // Label to mark the end of the loop
                    self.labels.insert(end_label.clone(), self.bytecode.len());
                    self.bytecode.push(Bytecode::Label(end_label.clone()));

                    // Resolve the conditional jump to the end of the loop
                    match label_to {
                        Some(l) => self.jumps.push((jump_if_false_pos, l.clone())),
                        None => self.jumps.push((jump_if_false_pos, end_label.clone())),
                    }

                    // // Handle label_to (optional jump after loop finishes)
                    // if let Some(label_to) = label_to {
                    //     let jump_pos = self.bytecode.len();
                    //     self.bytecode.push(Bytecode::Jump(0)); // Placeholder
                    //     self.jumps.push((jump_pos, label_to.clone()));
                    // }

                    // Link the label_until to the end of the loop
                    self.labels.insert(label_until.clone(), self.bytecode.len());

                    // Restore the previous loop context
                    self.loop_context.pop();
                } else {
                    // Handle Address case
                    todo!();
                }
            }
            OneLineStatementKind::UnconditionalJump { label } => {
                let jump_pos = self.bytecode.len();
                self.bytecode.push(Bytecode::Jump(0)); // Placeholder
                self.jumps.push((jump_pos, label.clone()));

            }
            // Handling other OneLineStatementKind cases...
            _ => {}
        }
    }

    fn visit_simple_statement(&mut self, statement: &SimpleStatement) {
        statement.node.accept(self);
    }

    fn visit_simple_statement_kind(&mut self, kind: &SimpleStatementKind) {
        match kind {
            SimpleStatementKind::Assign { lhs, rhs } => {
                rhs.accept(self);
                if let ExpressionKind::Var { name } = &lhs.node {
                    self.bytecode.push(Bytecode::SetVar(name.clone()));
                }
            }
            SimpleStatementKind::Expression { expression } => expression.accept(self),
            SimpleStatementKind::Import { labels, path, alias } => todo!(),
            SimpleStatementKind::Send { lhs, rhs } => {
                rhs.accept(self);
                lhs.accept(self);
                self.bytecode.push(Bytecode::Alloc);
            },
            SimpleStatementKind::Exchange { lhs, rhs } => todo!(),
        }
    }

    fn visit_expression(&mut self, expression: &Expression) {
        expression.node.accept(self);
    }

    fn visit_expression_kind(&mut self, kind: &ExpressionKind) {
        match kind {
            ExpressionKind::Int { value } => self
                .bytecode
                .push(Bytecode::Constant(Value::new_int(*value))),
            ExpressionKind::Var { name } => self.bytecode.push(Bytecode::GetVar(name.clone())),
            ExpressionKind::BinaryOp { op, lhs, rhs } => {
                lhs.accept(self);
                rhs.accept(self);
                match op {
                    BinaryOp::Sum => self.bytecode.push(Bytecode::Add),
                    BinaryOp::Sub => self.bytecode.push(Bytecode::Sub),
                    BinaryOp::Mul => self.bytecode.push(Bytecode::Mul),
                    BinaryOp::Div => self.bytecode.push(Bytecode::Div),
                    BinaryOp::Mod => self.bytecode.push(Bytecode::Mod),
                    BinaryOp::And => self.bytecode.push(Bytecode::And),
                    BinaryOp::Or => self.bytecode.push(Bytecode::Or),
                    BinaryOp::EQ => self.bytecode.push(Bytecode::Equal),
                    BinaryOp::NE => self.bytecode.push(Bytecode::NotEqual), // Example only
                    BinaryOp::GT => self.bytecode.push(Bytecode::Greater),
                    BinaryOp::LT => self.bytecode.push(Bytecode::Less),
                }
            }
            ExpressionKind::Null => self.bytecode.push(Bytecode::Constant(Value::Null)),
            ExpressionKind::Float { value } => self
                .bytecode
                .push(Bytecode::Constant(Value::new_float(*value))),
            ExpressionKind::Bool { value } => self
                .bytecode
                .push(Bytecode::Constant(Value::new_bool(*value))),
            ExpressionKind::String { value } => self
                .bytecode
                .push(Bytecode::Constant(Value::new_string(value.to_string()))),
            ExpressionKind::List { elements } => todo!(),
            ExpressionKind::Call { function, args } => {
                for arg in args {
                    arg.accept(self);
                    self.bytecode
                        .push(Bytecode::Call(function.to_string(), args.len()))
                }
            }
            ExpressionKind::UnaryOp { op, expr } => {
                expr.accept(self);
                match op {
                    UnaryOp::Dereference => self.bytecode.push(Bytecode::Deref),
                    UnaryOp::MultipleDereference(expr) => {
                        expr.accept(self);
                        self.bytecode.push(Bytecode::MulDeref)
                    }
                    UnaryOp::Not => self.bytecode.push(Bytecode::Negate),
                };
            }
        }
    }
}
