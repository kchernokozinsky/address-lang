use common::typings::Type;
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
            ValueError::_UnexpectedType {
                expected_types,
                actual_type,
                actual_value,
            } => write!(
                f,
                "Expect types '{:?}', but actual : ({}: {})",
                expected_types, actual_type, actual_value
            ),
            // ... handle other errors ...
        }
    }
}

impl std::error::Error for ValueError {}