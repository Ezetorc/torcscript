use crate::errors::lang_error::LangError;

#[derive(Debug)]
pub enum InterpreterError {
    NotImplemented(String),
}

impl From<InterpreterError> for LangError {
    fn from(error: InterpreterError) -> Self {
        LangError::Interpreter(error)
    }
}
