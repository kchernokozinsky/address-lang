use parser::ast::{visitor::Visitor, *};
use value::Value;

use crate::bytecode::Bytecode;

#[derive(Default)]
pub struct BytecodeGenerator {
    bytecode: Vec<Bytecode>,
}

impl BytecodeGenerator {
    pub fn new() -> Self {
        Self {
            bytecode: Vec::new(),
        }
    }

    pub fn get_bytecode(self) -> Vec<Bytecode> {
        self.bytecode
    }
}

impl Visitor for BytecodeGenerator {
    fn visit_algorithm(&mut self, algorithm: &Algorithm) {
        match algorithm {
            Algorithm::Body(lines) => {
                for line in lines {
                    line.accept(self);
                }
            }
        }
    }

    fn visit_file_line(&mut self, file_line: &FileLine) {
        match file_line {
            FileLine::Line { statements, .. } => {
                statements.accept(self);
            }
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
            OneLineStatementKind::SubProgram { sp_name, args, .. } => {
                for arg in args {
                    arg.accept(self);
                }
                self.bytecode
                    .push(Bytecode::Call(sp_name.identifier.clone(), args.len()));
            }
            OneLineStatementKind::Loop {
                initial_value,
                step,
                last_value_or_condition,
                iterator,
                ..
            } => {
                //
            }
            OneLineStatementKind::Predicate {
                condition,
                if_true,
                if_false,
            } => {
                condition.accept(self);
                if_true.accept(self);
                if_false.accept(self);
            }
            OneLineStatementKind::Exit => self.bytecode.push(Bytecode::Return),
            OneLineStatementKind::Return => self.bytecode.push(Bytecode::Return),
            OneLineStatementKind::UnconditionalJump { label } => todo!(),
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
            _ => {}
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
                    BinaryOp::NE => self.bytecode.push(Bytecode::Equal), // Example only
                    BinaryOp::GT => self.bytecode.push(Bytecode::Greater),
                    BinaryOp::LT => self.bytecode.push(Bytecode::Less),
                }
            }
            _ => {}
        }
    }
}
