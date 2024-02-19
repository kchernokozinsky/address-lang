use colored::*;
pub struct EvaluationErrorPrinter {
    source_text: String,
}

impl EvaluationErrorPrinter {

    pub fn new(source_text: String) -> Self {
        EvaluationErrorPrinter { source_text }
    }

    pub fn print_error(&self, error: &EvaluationError) {
        match error {
            EvaluationError::SyntaxError(start_loc, end_loc, message) => {
                self.print_error_message(start_loc, end_loc, message, "error");
            },
            EvaluationError::TypeError(start_loc, end_loc, message) => {
                self.print_error_message(start_loc, end_loc, message, "type error");
            },
            EvaluationError::RuntimeError(start_loc, end_loc, runtime_error) => {
                let message = format!("{}", runtime_error); // Assuming RuntimeError implements Display
                self.print_error_message(start_loc, end_loc, &message, "runtime error");
            },
            EvaluationError::UnhandledStatement(start_loc, end_loc, kind) => {
                let message = format!("unhandled statement: {:?}", kind); // Assuming kind is Debug-printable
                self.print_error_message(start_loc, end_loc, &message, "unhandled statement");
            },
            EvaluationError::UnhandledExpression(start_loc, end_loc, kind) => {
                let message = format!("unhandled expression: {:?}", kind);
                self.print_error_message(start_loc, end_loc, &message, "unhandled expression");
            },
            // Extend this pattern for other variants...
            _ => println!("{}", "Unhandled error variant".red()),
        }
    }

    fn print_error_message(&self, start_loc: &Location, end_loc: &Location, message: &str, error_type: &str) {
        if let Ok(code_line) = self.get_code_snippet(start_loc.row()) {
            let indent = " ".repeat(start_loc.row().to_string().len() + 1);
            let error_message = format!("\n{}: {}", error_type.red().bold(), message.red());
            let location_indicator = format!("{}--> {}:{} .. {}:{}",indent, start_loc.row(), start_loc.column(), end_loc.row(), end_loc.column()).blue();
            let code_snippet = code_line.trim_end();

            let end_column: usize = if end_loc.column() == 0 {code_line.len() + 1} else {end_loc.column()};
            // print error and location indicator
            println!("{}\n{}", error_message, location_indicator);
            println!("{}|\n{} | {}", indent, start_loc.row(), code_snippet);

            let underline = " ".repeat(start_loc.column()) + &"^".repeat((end_column.saturating_sub(start_loc.column())).max(1)).red().to_string();
            println!("{}|{}", indent, underline);

        } else {
            println!("Error locating source code for row {}", start_loc.row());
        }
    }

    fn get_code_snippet(&self, line_number: usize) -> Result<String, &'static str> {
        self.source_text
            .lines()
            .nth(line_number.saturating_sub(1)) // Account for zero-based indexing of nth
            .map(|line| line.to_string())
            .ok_or("Line not found")
    }
}