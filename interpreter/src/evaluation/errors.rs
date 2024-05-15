use crate::evaluation::*;
use common::location::Location;
use value::error::ValueError;

pub enum RuntimeError {
    NullReference,
    DivisionByZero,
    TypeError(ValueError),
    IndexOutOfBounds(usize, usize),
    VariableNotFound(String),
    LabelNotFound(String),
    LabelAlreadyRegistered(String, usize, usize),
    FunctionNotFound(String),
    InvalidArgument(String),
    FunctionCallError(String, String),
    InvalidArgumentsNumber(String, usize, usize),
    // ...other runtime errors
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::NullReference => write!(f, "Null reference error"),
            RuntimeError::DivisionByZero => write!(f, "Division by zero error"),
            RuntimeError::TypeError(error) => write!(f, "Type error: {}", error),
            RuntimeError::IndexOutOfBounds(index, length) => write!(
                f,
                "Index out of bounds error: index {} in a collection of length {}",
                index, length
            ),
            RuntimeError::VariableNotFound(var_name) => {
                write!(f, "Variable '{}' not found", var_name)
            }
            RuntimeError::LabelNotFound(label_name) => {
                write!(f, "Label '{}' not found", label_name)
            }
            RuntimeError::FunctionNotFound(func_name) => {
                write!(f, "Function '{}' not found", func_name)
            }
            RuntimeError::InvalidArgument(arg_desc) => {
                write!(f, "Invalid argument error: {}", arg_desc)
            }
            RuntimeError::FunctionCallError(func_name, error) => {
                write!(f, "Function '{}' raised error: '{}'", func_name, error)
            }
            RuntimeError::InvalidArgumentsNumber(sp_name, expected_number, actual_number) => {
                write!(
                    f,
                    "Invalid arguments number error: subprogram:  '{}', expected: {} , actual: {}",
                    sp_name, expected_number, actual_number
                )
            }
            RuntimeError::LabelAlreadyRegistered(label_name, registered_line, try_line) => {
                write!(
                    f,
                    "Label '{}' can't be registered twice at line {}. It was registered at line {}",
                    label_name, try_line, registered_line
                )
            }
        }
    }
}

impl std::error::Error for RuntimeError {}

impl std::fmt::Debug for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <RuntimeError as std::fmt::Display>::fmt(self, f)
    }
}

// EvaluationError enum
pub enum EvaluationError {
    SyntaxError(Location, Location, String),
    TypeError(Location, Location, String),
    RuntimeError(Location, Location, RuntimeError),
    RuntimeErrorWithoutLocation(RuntimeError), // Integrating RuntimeError
    UnhandledStatement(Location, Location, SimpleStatementKind),
    UnhandledFormula(Location, Location, OneLineStatementKind),
    UnhandledExpression(Location, Location, ExpressionKind), // ...other errors
    SubProgramDeclaration(Location, Location, String),
    SubProgram(Location, Location, RuntimeError), // ...other errors
}

impl std::fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvaluationError::SyntaxError(left_loc, right_loc, message) => write!(
                f,
                "Syntax error between {:?} and {:?}: {}",
                left_loc, right_loc, message
            ),
            EvaluationError::TypeError(left_loc, right_loc, message) => write!(
                f,
                "Type error between {:?} and {:?}: {}",
                left_loc, right_loc, message
            ),
            EvaluationError::RuntimeError(left_loc, right_loc, runtime_error) => write!(
                f,
                "Runtime Error between {:?} and {:?}: {}",
                left_loc, right_loc, runtime_error
            ),
            EvaluationError::UnhandledStatement(left_loc, right_loc, kind) => write!(
                f,
                "Unhandled statement between {:?} and {:?}: {:?}",
                left_loc, right_loc, kind
            ),
            EvaluationError::UnhandledExpression(left_loc, right_loc, kind) => write!(
                f,
                "Unhandled expression between {:?} and {:?}: {:?}",
                left_loc, right_loc, kind
            ),

            EvaluationError::UnhandledFormula(left_loc, right_loc, kind) => write!(
                f,
                "Unhandled statement between {:?} and {:?}: {:?}",
                left_loc, right_loc, kind
            ),
            EvaluationError::SubProgramDeclaration(left_loc, right_loc, sp_name) => write!(
                f,
                "Wrong subprogram declaration between {:?} and {:?} in subprogram '{}'",
                left_loc, right_loc, sp_name
            ),
            EvaluationError::SubProgram(left_loc, right_loc, runtime_error) => write!(
                f,
                "SubProgram Error between {:?} and {:?}: {}",
                left_loc, right_loc, runtime_error
            ),
            EvaluationError::RuntimeErrorWithoutLocation(runtime_error) => {
                write!(f, "Runtime Error : {}", runtime_error)
            }
        }
    }
}

impl std::error::Error for EvaluationError {}

impl std::fmt::Debug for EvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <EvaluationError as std::fmt::Display>::fmt(self, f)
    }
}

//---

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
            }
            EvaluationError::TypeError(start_loc, end_loc, message) => {
                self.print_error_message(start_loc, end_loc, message, "type error");
            }
            EvaluationError::RuntimeError(start_loc, end_loc, runtime_error) => {
                let message = format!("{}", runtime_error); // Assuming RuntimeError implements Display
                self.print_error_message(start_loc, end_loc, &message, "runtime error");
            }
            EvaluationError::UnhandledStatement(start_loc, end_loc, kind) => {
                let message = format!("unhandled statement: {:?}", kind); // Assuming kind is Debug-printable
                self.print_error_message(start_loc, end_loc, &message, "unhandled statement");
            }
            EvaluationError::UnhandledExpression(start_loc, end_loc, kind) => {
                let message = format!("unhandled expression: {:?}", kind);
                self.print_error_message(start_loc, end_loc, &message, "unhandled expression");
            }
            // Extend this pattern for other variants...
            _ => println!("{}", "Unhandled error variant".red()),
        }
    }

    fn print_error_message(
        &self,
        start_loc: &Location,
        end_loc: &Location,
        message: &str,
        error_type: &str,
    ) {
        if let Ok(code_line) = self.get_code_snippet(start_loc.row()) {
            let indent = " ".repeat(start_loc.row().to_string().len() + 1);
            let error_message = format!("\n{}: {}", error_type.red().bold(), message.red());
            let location_indicator = format!(
                "{}--> {}:{} .. {}:{}",
                indent,
                start_loc.row(),
                start_loc.column(),
                end_loc.row(),
                end_loc.column()
            )
            .blue();
            let code_snippet = code_line.trim_end();

            let end_column: usize = if end_loc.column() == 0 {
                code_line.len() + 1
            } else {
                end_loc.column()
            };
            // print error and location indicator
            println!("{}\n{}", error_message, location_indicator);
            println!("{}|\n{} | {}", indent, start_loc.row(), code_snippet);

            let underline = " ".repeat(start_loc.column())
                + &"^"
                    .repeat((end_column.saturating_sub(start_loc.column())).max(1))
                    .red()
                    .to_string();
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
