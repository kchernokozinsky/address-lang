use value::error::ValueError;

#[derive(Debug)]
pub enum VMError {
    StackUnderflow,
    InvalidAddress,
    InvalidOperation,
    UndefinedFunction(String),
    UnexpectedType(ValueError),
    Custom(String),
}

impl From<ValueError> for VMError {
    fn from(err: ValueError) -> Self {
        VMError::UnexpectedType(err)
    }
}