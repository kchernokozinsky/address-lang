use value::error::ValueError;

use crate::{heap::HeapError, scope::ScopeError};

#[derive(Debug)]
pub enum VMError {
    StackUnderflow,
    InvalidAddress,
    InvalidOperation,
    UndefinedFunction(String),
    UnexpectedType(ValueError),
    HeapEror(HeapError),
    ScopeError(ScopeError),
    Custom(String),
}

impl From<ValueError> for VMError {
    fn from(err: ValueError) -> Self {
        VMError::UnexpectedType(err)
    }
}

impl From<HeapError> for VMError {
    fn from(err: HeapError) -> Self {
        VMError::HeapEror(err)
    }
}

impl From<ScopeError> for VMError {
    fn from(err: ScopeError) -> Self {
        VMError::ScopeError(err)
    }
}
