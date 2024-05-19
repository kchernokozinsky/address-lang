use super::*;

pub trait Visitor {
    fn visit_algorithm(&mut self, algorithm: &Algorithm);
    fn visit_file_line(&mut self, file_line: &FileLine);
    fn visit_statements(&mut self, statements: &Statements);
    fn visit_one_line_statement(&mut self, statement: &OneLineStatement);
    fn visit_one_line_statement_kind(&mut self, kind: &OneLineStatementKind);
    fn visit_simple_statement(&mut self, statement: &SimpleStatement);
    fn visit_simple_statement_kind(&mut self, kind: &SimpleStatementKind);
    fn visit_expression(&mut self, expression: &Expression);
    fn visit_expression_kind(&mut self, kind: &ExpressionKind);
}

impl Algorithm {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_algorithm(self);
    }
}

impl FileLine {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_file_line(self);
    }
}

impl Statements {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_statements(self);
    }
}

impl OneLineStatement {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_one_line_statement(self);
    }
}

impl OneLineStatementKind {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_one_line_statement_kind(self);
    }
}

impl SimpleStatement {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_simple_statement(self);
    }
}

impl SimpleStatementKind {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_simple_statement_kind(self);
    }
}

impl Expression {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_expression(self);
    }
}

impl ExpressionKind {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_expression_kind(self);
    }
}
