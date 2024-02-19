use std::{fmt::{Display, Formatter, self}, error::Error};
use crate::location::Location;


pub enum LexError {
    Unexpected(Location, char),
    UnterminatedStringLiteral(Location),
    FloatFormatError(Location, String),
    IntegerFormatError(Location, String),
}


impl Display for LexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LexError::Unexpected(loc, ch) => 
                write!(f, "Unexpected character '{}' at {:?}", ch, loc),
            LexError::UnterminatedStringLiteral(loc) => 
                write!(f, "Unterminated string literal at {:?}", loc),
            LexError::FloatFormatError(loc, err) => 
                write!(f, "Float format error '{}' at {:?}", err, loc),
            LexError::IntegerFormatError(loc, err) => 
                write!(f, "Integer format error '{}' at {:?}", err, loc),
        }
    }
}

impl Error for LexError {}

impl std::fmt::Debug for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <LexError as Display>::fmt(self, f)
    }
}
