use crate::location::Location;
use crate::evaluation::*;

pub enum RuntimeError {
    NullReference(Location, Location),
    DivisionByZero(Location, Location),
    TypeError(Location, Location, String),
    IndexOutOfBounds(Location, Location, usize, usize),
    VariableNotFound(Location, Location, String),
    LabelNotFound(Location, Location, String),
    FunctionNotFound(Location, Location, String),
    InvalidArgument(Location, Location, String),
    FunctionCallError(Location, Location, String, String),
    // ...other runtime errors
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::DivisionByZero(left_loc, right_loc) => 
                write!(f, "Division by zero error\n{:?} ... {:?}", left_loc, right_loc),
            RuntimeError::NullReference(left_loc, right_loc) => 
                write!(f, "Null reference error\n{:?} ... {:?}", left_loc, right_loc),
            RuntimeError::TypeError(left_loc, right_loc, message) => 
                write!(f, "Type error\n{:?} ... {:?}\ndetailed info: {}", left_loc, right_loc, message),
            RuntimeError::IndexOutOfBounds(left_loc, right_loc, index, length) => 
                write!(f, "Index out of bounds error\n{:?} ... {:?}\ndetailed info: index {} in a collection of length {}", left_loc, right_loc, index, length),
            RuntimeError::VariableNotFound(left_loc, right_loc, var_name) => 
                write!(f, "Variable '{}' not found\n{:?} ... {:?}", var_name, left_loc, right_loc),
            RuntimeError::FunctionNotFound(left_loc, right_loc, func_name) => 
                write!(f, "Function '{}' not found\n{:?} ... {:?}", func_name, left_loc, right_loc),
            RuntimeError::InvalidArgument(left_loc, right_loc, arg_desc) => 
                write!(f, "Invalid argument error\n{:?} ... {:?}: {}", left_loc, right_loc, arg_desc),
            RuntimeError::LabelNotFound(left_loc, right_loc, label_name) => 
            write!(f, "Label '{}' not found\n{:?} ... {:?}", label_name, left_loc, right_loc),
            RuntimeError::FunctionCallError(left_loc, right_loc, func_name, error) => 
            write!(f, "Function '{}' raise error: '{}'\n{:?} ... {:?}", func_name, error, left_loc, right_loc),
            // ...other runtime errors
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
    RuntimeError(RuntimeError), // Integrating RuntimeError
    UnhandledStatement(Location, Location, SimpleStatementKind),
    UnhandledFormula(Location, Location, OneLineStatementKind),
    UnhandledExpression(Location, Location, ExpressionKind),
    UnhandledBinaryOperation(Location, Location, BinaryOp),
    // ...other errors
}

impl std::fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvaluationError::SyntaxError(left_loc, right_loc, message) => 
                write!(f, "Syntax error between {:?} and {:?}: {}", left_loc, right_loc, message),
            EvaluationError::TypeError(left_loc, right_loc, message) => 
                write!(f, "Type error between {:?} and {:?}: {}", left_loc, right_loc, message),
            EvaluationError::RuntimeError(runtime_error) => 
                write!(f, "Runtime Error\n{}", runtime_error),
            EvaluationError::UnhandledStatement(left_loc, right_loc, kind) => 
                write!(f, "Unhandled statement between {:?} and {:?}: {:?}", left_loc, right_loc, kind),
            EvaluationError::UnhandledExpression(left_loc, right_loc, kind) => 
                write!(f, "Unhandled expression between {:?} and {:?}: {:?}", left_loc, right_loc, kind),
            EvaluationError::UnhandledBinaryOperation(left_loc, right_loc, kind) => 
            write!(f, "Unhandled Binary Operation between {:?} and {:?}: {:?}", left_loc, right_loc, kind),
            EvaluationError::UnhandledFormula(left_loc, right_loc, kind) => 
            write!(f, "Unhandled statement between {:?} and {:?}: {:?}", left_loc, right_loc, kind),
            // ...other errors
        }
    }
}

impl std::error::Error for EvaluationError {}

impl std::fmt::Debug for EvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <EvaluationError as std::fmt::Display>::fmt(self, f)
    }
}