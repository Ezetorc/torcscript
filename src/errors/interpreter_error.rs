use std::fmt::Display;

use crate::errors::lang_error::LangError;

#[derive(Debug)]
pub enum InterpreterError {
    TypeMismatch(String),
    NotFound(String),
    DivisionByZero(String),
    InvalidOperator(String),
    InvalidAmount(String),
}

impl From<InterpreterError> for LangError {
    fn from(error: InterpreterError) -> Self {
        LangError::Interpreter(error)
    }
}

impl Display for InterpreterError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TypeMismatch(error)
            | Self::NotFound(error)
            | Self::DivisionByZero(error)
            | Self::InvalidOperator(error)
            | Self::InvalidAmount(error) => write!(formatter, "{error}"),
        }
    }
}
