use std::fmt;

use crate::errors::lang_error::LangError;

#[derive(Debug)]
pub enum InterpreterError {
    TypeMismatch(String),
    NotFound(String),
    DivisionByZero(String),
    InvalidOperator(String),
    InvalidAmount(String),
    InvalidAssignment(String),
    NotCallable(String),
}

impl From<InterpreterError> for LangError {
    fn from(error: InterpreterError) -> Self {
        LangError::Interpreter(error)
    }
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TypeMismatch(error)
            | Self::NotFound(error)
            | Self::DivisionByZero(error)
            | Self::InvalidOperator(error)
            | Self::InvalidAmount(error)
            | Self::InvalidAssignment(error)
            | Self::NotCallable(error) => write!(formatter, "{error}"),
        }
    }
}
