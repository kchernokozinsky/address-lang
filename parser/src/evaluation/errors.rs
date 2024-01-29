use crate::evaluation::*;
use crate::location::Location;
use crate::typings::Type;

#[derive(Debug)]
pub enum ValueError {
    IncompatibleTypes {
        operation: String,
        lhs_type: Type,
        lhs_value: String,
        rhs_type: Type,
        rhs_value: String,
    },
    UnexpectedType {
        expected_type: Type,
        actual_type: Type,
        actual_value: String,
    }, // ... other error types can be added here ...
    _UnexpectedType {
        expected_types: Vec<Type>,
        actual_type: Type,
        actual_value: String,
    },
}

impl std::fmt::Display for ValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueError::IncompatibleTypes {
                operation,
                lhs_type,
                lhs_value,
                rhs_type,
                rhs_value,
            } => write!(
                f,
                "Incompatible types for '{}': ({}: {}) and ({}: {})",
                operation, lhs_type, lhs_value, rhs_type, rhs_value
            ),
            ValueError::UnexpectedType {
                expected_type,
                actual_type,
                actual_value,
            } => write!(
                f,
                "Expect type '{}', but actual : ({}: {})",
                expected_type, actual_type, actual_value
            ),
            ValueError::_UnexpectedType { expected_types, actual_type, actual_value } => write!(
                f,
                "Expect types '{:?}', but actual : ({}: {})",
                expected_types, actual_type, actual_value
            ),
            // ... handle other errors ...
        }
    }
}

impl std::error::Error for ValueError {}

pub enum RuntimeError {
    NullReference,
    DivisionByZero,
    TypeError(ValueError),
    IndexOutOfBounds(usize, usize),
    VariableNotFound(String),
    LabelNotFound(String),
    FunctionNotFound(String),
    InvalidArgument(String),
    FunctionCallError(String, String),
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
    RuntimeError(Location, Location, RuntimeError), // Integrating RuntimeError
    UnhandledStatement(Location, Location, SimpleStatementKind),
    UnhandledFormula(Location, Location, OneLineStatementKind),
    UnhandledExpression(Location, Location, ExpressionKind),
    UnhandledBinaryOperation(Location, Location, BinaryOp),
    // ...other errors
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
            EvaluationError::UnhandledBinaryOperation(left_loc, right_loc, kind) => write!(
                f,
                "Unhandled Binary Operation between {:?} and {:?}: {:?}",
                left_loc, right_loc, kind
            ),
            EvaluationError::UnhandledFormula(left_loc, right_loc, kind) => write!(
                f,
                "Unhandled statement between {:?} and {:?}: {:?}",
                left_loc, right_loc, kind
            ),
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
