use crate::errors::lang_error::LangError;

#[derive(Debug)]
pub enum InterpreterError {
    TypeMismatch(String),
    NotFound(String),
    DivisionByZero(String),
}

impl From<InterpreterError> for LangError {
    fn from(error: InterpreterError) -> Self {
        LangError::Interpreter(error)
    }
}
