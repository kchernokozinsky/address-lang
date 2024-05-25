use std::collections::HashMap;

use parser::ast::{visitor::Visitor, *};
use value::Value;

use crate::bytecode::Bytecode;
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
                    Bytecode::CallSubProgram(offset, arity) => {
                        Bytecode::CallSubProgram(offset + address, arity)
                    }
                    _ => todo!(),
                }
            } else {
                panic!("Undefined label: {}", label);
            }
        }
    }

    fn visit_loop_body(&mut self, label_until: &str) {
        match self.ast {
            Algorithm::Body(lines) => {
                while self.current_position < lines.len() {
                    let line = &lines[self.current_position];
                    match line {
                        FileLine::Line { labels, statements } => {
                            for label in labels {
                                self.labels.insert(label.clone(), self.bytecode.len());
                                self.bytecode.push(Bytecode::Label(label.clone()));
                            }
                            if labels.contains(&label_until.to_string()) {
                                break;
                            }
                            statements.accept(self);
                        }
                    }
                    self.current_position += 1;
                }
            }
        }
    }

    fn bind_names(&mut self, name: &str, arity: usize) -> Vec<String> {
        let mut local_names: Vec<String> = vec![];
        match self.ast {
            Algorithm::Body(lines) => {
                for FileLine::Line { labels, statements } in lines {
                    if labels.contains(&name.to_string()) {
                        match statements {
                            Statements::OneLineStatement(_) => todo!(),
                            Statements::SimpleStatements(stmts) => {
                                for stmt in stmts.into_iter() {
                                    match &stmt.node {
                                        SimpleStatementKind::Send { lhs, rhs } => match &lhs.node {
                                            ExpressionKind::Var { name } => {
                                                local_names.push(name.to_string())
                                            }
                                            _ => todo!(),
                                        },
                                        _ => todo!(),
                                    }
                                }
                                // self.bytecode.push(Bytecode::BindArgs(local_names.clone()));
                                return local_names;
                            }
                        }
                    }
                }
            }
        }
        return local_names;
    }

    fn generate_list(&mut self, elements: &[Box<Expression>]) {
        if elements.is_empty() {
            self.bytecode.push(Bytecode::Constant(Value::Null));
            return;
        }
        // allocate last element
        // self.bytecode.push(Bytecode::Store);
        self.bytecode.push(Bytecode::Constant(Value::Null));
        self.bytecode.push(Bytecode::StoreAddr); // stack: address to last element next address
        elements.last().unwrap().accept(self);
        self.bytecode.push(Bytecode::Alloc);
        self.bytecode.push(Bytecode::Store);

        let tail = &elements[0..elements.len() - 1];
        for e in tail.iter().rev() {
            self.bytecode.push(Bytecode::StoreAddr);
            e.accept(self);
            self.bytecode.push(Bytecode::Alloc); // stack:  address to last element value, address to previos element value
                                                 // self.bytecode.push(Bytecode::Swap);           // stack: address to previos element value, stack: address to last element value
            self.bytecode.push(Bytecode::Store); // stack: address to previos element value
                                                 //  stack: address to last element value
        }
    }
}

impl<'a> Visitor for BytecodeGenerator<'a> {
    fn visit_algorithm(&mut self, algorithm: &Algorithm) {
        match algorithm {
            Algorithm::Body(lines) => {
                self.current_position = 0;
                while self.current_position < lines.len() {
                    lines[self.current_position].accept(self);
                    self.current_position += 1;
                }
            }
        }
    }

    fn visit_file_line(&mut self, file_line: &FileLine) {
        match file_line {
            FileLine::Line { labels, statements } => {
                for label in labels {
                    self.labels.insert(label.clone(), self.bytecode.len());
                    self.bytecode.push(Bytecode::Label(label.clone()));
                }
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
                    self.bytecode.push(Bytecode::StoreVar(name.clone()));
                    self.bytecode.push(Bytecode::LoadVar(name.clone()));
                    self.bytecode.push(Bytecode::Store);
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
                    self.bytecode.push(Bytecode::LoadVar(name.clone()));
                    self.bytecode.push(Bytecode::Deref);
                    self.bytecode.push(Bytecode::Add);
                    self.bytecode.push(Bytecode::LoadVar(name.clone()));
                    self.bytecode.push(Bytecode::Store);
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
            OneLineStatementKind::SubProgram {
                sp_name,
                args,
                label_to,
            } => {
                // Generate bytecode for arguments

                for arg in args {
                    arg.accept(self);
                }

                self.bytecode.push(Bytecode::PushScope);

                let local_variables = self.bind_names(&sp_name.identifier, args.len());
                for local_var in local_variables.iter().rev() {
                    self.bytecode
                        .push(Bytecode::BindAddr(local_var.to_string()));
                }

                // Call the subprogram
                let jump_pos = self.bytecode.len();
                self.jumps.push((jump_pos, sp_name.identifier.clone()));

                self.bytecode
                    .push(Bytecode::CallSubProgram(args.len() * 3 + 1, args.len()));

                self.bytecode.push(Bytecode::PopScope);

                let call_declaration_label =
                    format!("call_declaration_label_{}", self.bytecode.len());
                self.labels
                    .insert(call_declaration_label.clone(), self.bytecode.len());

                self.bytecode
                    .push(Bytecode::Label(call_declaration_label.clone()));
            }
            OneLineStatementKind::Predicate {
                condition,
                if_true,
                if_false,
            } => {
                // Evaluate the condition
                condition.accept(self);

                // Generate a placeholder jump for false condition
                let jump_if_false_pos = self.bytecode.len();
                self.bytecode.push(Bytecode::JumpIfFalse(0)); // Placeholder

                // Generate bytecode for the true branch
                if_true.accept(self);

                // Generate a jump to skip the false branch
                let jump_to_end_pos = self.bytecode.len();
                self.bytecode.push(Bytecode::Jump(0)); // Placeholder

                // Set the placeholder for the false branch
                let false_branch_pos = self.bytecode.len();
                self.bytecode[jump_if_false_pos] = Bytecode::JumpIfFalse(false_branch_pos);

                // Generate bytecode for the false branch if it exists
                if_false.accept(self);

                // Set the placeholder for the end of the false branch
                let end_pos = self.bytecode.len();
                self.bytecode[jump_to_end_pos] = Bytecode::Jump(end_pos);
            }
            OneLineStatementKind::Exit => self.bytecode.push(Bytecode::Halt),
            OneLineStatementKind::Return => self.bytecode.push(Bytecode::Return),
            // Handling other OneLineStatementKind cases...
        }
    }

    fn visit_simple_statement(&mut self, statement: &SimpleStatement) {
        statement.node.accept(self);
    }

    fn visit_simple_statement_kind(&mut self, kind: &SimpleStatementKind) {
        match kind {
            SimpleStatementKind::Assign { lhs, rhs } => {
                rhs.accept(self);
                match &lhs.node {
                    ExpressionKind::Null => todo!(),
                    ExpressionKind::Float { value } => todo!(),
                    ExpressionKind::Bool { value } => todo!(),
                    ExpressionKind::Int { value } => todo!(),
                    ExpressionKind::String { value } => todo!(),
                    ExpressionKind::Var { name } => {
                        self.bytecode.push(Bytecode::BindAddr(name.to_string()));
                    }
                    ExpressionKind::List { elements } => todo!(),
                    ExpressionKind::Call { .. } => todo!(),
                    ExpressionKind::UnaryOp { op, expr } => match op {
                        UnaryOp::Dereference => {
                            expr.accept(self);
                            self.bytecode.push(Bytecode::Store);
                        }
                        UnaryOp::MultipleDereference(n) => {
                            expr.accept(self);
                            n.accept(self);
                            self.bytecode.push(Bytecode::Constant(Value::new_int(1)));
                            self.bytecode.push(Bytecode::Sub);
                            self.bytecode.push(Bytecode::MulDeref);
                            self.bytecode.push(Bytecode::Store);
                        }
                        UnaryOp::Not => todo!(),
                        UnaryOp::Minus => todo!(),
                    },
                    ExpressionKind::BinaryOp { .. } => todo!(),
                }
            }
            SimpleStatementKind::Expression { expression } => expression.accept(self),
            SimpleStatementKind::Import {
                labels,
                path,
                alias,
            } => todo!(),
            SimpleStatementKind::Send { lhs, rhs } => {
                rhs.accept(self);
                lhs.accept(self);
                self.bytecode.push(Bytecode::Store);
            }
            SimpleStatementKind::Exchange { lhs, rhs } => todo!(),
            SimpleStatementKind::Del { rhs } => {
                rhs.accept(self);
                self.bytecode.push(Bytecode::FreeAddr);
            }
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
            ExpressionKind::Var { name } => self.bytecode.push(Bytecode::LoadVar(name.clone())),
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
            ExpressionKind::List { elements } => self.generate_list(elements),
            ExpressionKind::Call { function, args } => {
                for arg in args {
                    arg.accept(self);
                }
                self.bytecode
                    .push(Bytecode::CallBuiltin(function.to_string(), args.len()))
            }
            ExpressionKind::UnaryOp { op, expr } => {
                expr.accept(self);
                match op {
                    UnaryOp::Dereference => self.bytecode.push(Bytecode::Deref),
                    UnaryOp::MultipleDereference(expr) => {
                        expr.accept(self);
                        self.bytecode.push(Bytecode::MulDeref)
                    }
                    UnaryOp::Not => self.bytecode.push(Bytecode::Not),
                    UnaryOp::Minus => self.bytecode.push(Bytecode::Negate),
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_visit_binary_op() {
        let source_text = "5 + 3";
        let algo: Algorithm = parser::parse(&source_text).unwrap();

        let mut generator = BytecodeGenerator::new(&algo);
        generator.visit_algorithm(&algo);

        let bytecode = generator.get_bytecode();
        println!("{:?}", bytecode);
        assert_eq!(
            bytecode,
            vec![
                Bytecode::Constant(Value::new_int(5)),
                Bytecode::Constant(Value::new_int(3)),
                Bytecode::Add
            ]
        );
    }

    #[test]
    fn test_visit_mulderef_op() {
        let source_text = "D {var, 4}";
        let algo: Algorithm = parser::parse(&source_text).unwrap();

        let mut generator = BytecodeGenerator::new(&algo);
        generator.visit_algorithm(&algo);

        let bytecode = generator.get_bytecode();
        println!("{:?}", bytecode);
        assert_eq!(
            bytecode,
            vec![
                Bytecode::LoadVar("var".to_string()),
                Bytecode::Constant(Value::Int(4)),
                Bytecode::MulDeref
            ]
        );
    }

    #[test]
    fn test_visit_deref_op() {
        let source_text = "'4";
        let algo: Algorithm = parser::parse(&source_text).unwrap();

        let mut generator = BytecodeGenerator::new(&algo);
        generator.visit_algorithm(&algo);

        let bytecode = generator.get_bytecode();
        println!("{:?}", bytecode);
        assert_eq!(
            bytecode,
            vec![Bytecode::Constant(Value::Int(4)), Bytecode::Deref]
        );
    }

    #[test]
    fn test_visit_not_op() {
        let source_text = "not true";
        let algo: Algorithm = parser::parse(&source_text).unwrap();

        let mut generator = BytecodeGenerator::new(&algo);
        generator.visit_algorithm(&algo);

        let bytecode = generator.get_bytecode();
        println!("{:?}", bytecode);
        assert_eq!(
            bytecode,
            vec![Bytecode::Constant(Value::new_bool(true)), Bytecode::Not]
        );
    }

    #[test]
    fn test_visit_if_else() {
        let source_text = "P { 5 < 3 } 1 | 2";
        let algo: Algorithm = parser::parse(&source_text).unwrap();

        let mut generator = BytecodeGenerator::new(&algo);
        generator.visit_algorithm(&algo);

        let bytecode = generator.get_bytecode();
        println!("{:?}", bytecode);
        assert_eq!(
            bytecode,
            vec![
                Bytecode::Constant(Value::new_int(5)),
                Bytecode::Constant(Value::new_int(3)),
                Bytecode::Less,
                Bytecode::JumpIfFalse(6),
                Bytecode::Constant(Value::new_int(1)),
                Bytecode::Jump(7),
                Bytecode::Constant(Value::new_int(2)),
            ]
        );
    }

    #[test]
    fn test_visit_loop() {
        let source_text = "
        L {1, 1, 'i < 6 => i} a
            Print {\"i: \", 'i}
        a ...";
        let algo: Algorithm = parser::parse(&source_text).unwrap();

        let mut generator = BytecodeGenerator::new(&algo);
        generator.visit_algorithm(&algo);

        let bytecode = generator.get_bytecode();
        println!("{:?}", bytecode);
        assert_eq!(
            bytecode,
            vec![
                Bytecode::Constant(Value::new_int(1)),
                Bytecode::StoreVar("i".to_string()),
                Bytecode::LoadVar("i".to_string()),
                Bytecode::Store,
                Bytecode::Label("loop_start_4".to_string()),
                Bytecode::LoadVar("i".to_string()),
                Bytecode::Deref,
                Bytecode::Constant(Value::new_int(6)),
                Bytecode::Less,
                Bytecode::JumpIfFalse(23),
                Bytecode::Label("loop_body_start_10".to_string()),
                Bytecode::Constant(Value::new_string("i: ".to_string())),
                Bytecode::LoadVar("i".to_string()),
                Bytecode::Deref,
                Bytecode::CallBuiltin("Print".to_string(), 2),
                Bytecode::Label("a".to_string()),
                Bytecode::Constant(Value::new_int(1)),
                Bytecode::LoadVar("i".to_string()),
                Bytecode::Deref,
                Bytecode::Add,
                Bytecode::LoadVar("i".to_string()),
                Bytecode::Store,
                Bytecode::Jump(4),
                Bytecode::Label("loop_end_10".to_string()),
            ]
        );
    }

    #[test]
    fn test_visit_subprogram() {
        let source_text = "
        l1 = [1,2,3];
        'k = 1
        SP get {l1, k, result}
        !
        get ... null => list; null => index; null => e
            'e = '(D {list, 'index} + 1)
        return";
        let algo: Algorithm = parser::parse(&source_text).unwrap();

        let mut generator = BytecodeGenerator::new(&algo);
        generator.visit_algorithm(&algo);

        let bytecode = generator.get_bytecode();
        println!("{:?}", bytecode);
        assert_eq!(
            bytecode,
            vec![
                Bytecode::Constant(Value::Null),
                Bytecode::StoreAddr,
                Bytecode::Constant(Value::new_int(3)),
                Bytecode::Alloc,
                Bytecode::Store,
                Bytecode::StoreAddr,
                Bytecode::Constant(Value::new_int(2)),
                Bytecode::Alloc,
                Bytecode::Store,
                Bytecode::StoreAddr,
                Bytecode::Constant(Value::new_int(1)),
                Bytecode::Alloc,
                Bytecode::Store,
                Bytecode::BindAddr("l1".to_string()),
                Bytecode::Constant(Value::new_int(1)),
                Bytecode::LoadVar("k".to_string()),
                Bytecode::Store,
                Bytecode::LoadVar("l1".to_string()),
                Bytecode::LoadVar("k".to_string()),
                Bytecode::LoadVar("result".to_string()),
                Bytecode::PushScope,
                Bytecode::BindAddr("e".to_string()),
                Bytecode::BindAddr("index".to_string()),
                Bytecode::BindAddr("list".to_string()),
                Bytecode::CallSubProgram(38, 3),
                Bytecode::PopScope,
                Bytecode::Label("call_declaration_label_26".to_string()),
                Bytecode::Halt,
                Bytecode::Label("get".to_string()),
                Bytecode::Constant(Value::Null),
                Bytecode::LoadVar("list".to_string()),
                Bytecode::Store,
                Bytecode::Constant(Value::Null),
                Bytecode::LoadVar("index".to_string()),
                Bytecode::Store,
                Bytecode::Constant(Value::Null),
                Bytecode::LoadVar("e".to_string()),
                Bytecode::Store,
                Bytecode::LoadVar("list".to_string()),
                Bytecode::LoadVar("index".to_string()),
                Bytecode::Deref,
                Bytecode::MulDeref,
                Bytecode::Constant(Value::new_int(1)),
                Bytecode::Add,
                Bytecode::Deref,
                Bytecode::LoadVar("e".to_string()),
                Bytecode::Store,
                Bytecode::Return,
            ]
        );
    }

    #[test]
    fn test_visit_list_allocation() {
        let source_text = "[1, 2, 3]";
        let algo: Algorithm = parser::parse(&source_text).unwrap();

        let mut generator = BytecodeGenerator::new(&algo);
        generator.visit_algorithm(&algo);

        let bytecode = generator.get_bytecode();
        println!("{:?}", bytecode);
        assert_eq!(
            bytecode,
            vec![
                Bytecode::Constant(Value::Null),
                Bytecode::StoreAddr,
                Bytecode::Constant(Value::new_int(3)),
                Bytecode::Alloc,
                Bytecode::Store,
                Bytecode::StoreAddr,
                Bytecode::Constant(Value::new_int(2)),
                Bytecode::Alloc,
                Bytecode::Store,
                Bytecode::StoreAddr,
                Bytecode::Constant(Value::new_int(1)),
                Bytecode::Alloc,
                Bytecode::Store,
            ]
        );
    }

    #[test]
    fn test_visit_assign_statement() {
        let source_text = "x = 10";
        let algo: Algorithm = parser::parse(&source_text).unwrap();

        let mut generator = BytecodeGenerator::new(&algo);
        generator.visit_algorithm(&algo);

        let bytecode = generator.get_bytecode();
        println!("{:?}", bytecode);
        assert_eq!(
            bytecode,
            vec![
                Bytecode::Constant(Value::new_int(10)),
                Bytecode::BindAddr("x".to_string())
            ]
        );
    }

    #[test]
    fn test_visit_logical_operations() {
        let source_text = "true and false or not true";
        let algo: Algorithm = parser::parse(&source_text).unwrap();

        let mut generator = BytecodeGenerator::new(&algo);
        generator.visit_algorithm(&algo);

        let bytecode = generator.get_bytecode();
        println!("{:?}", bytecode);
        assert_eq!(
            bytecode,
            vec![
                Bytecode::Constant(Value::new_bool(true)),
                Bytecode::Constant(Value::new_bool(false)),
                Bytecode::And,
                Bytecode::Constant(Value::new_bool(true)),
                Bytecode::Not,
                Bytecode::Or
            ]
        );
    }
}
