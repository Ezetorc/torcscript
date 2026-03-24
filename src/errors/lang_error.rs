use std::fmt;

use crate::errors::lexer_error::LexerError;

#[derive(Debug)]
pub enum LangError {
    Lexer(LexerError),
}

impl fmt::Display for LangError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LangError::Lexer(error) => write!(formatter, "Lexer error: {:?}", error),
        }
    }
}

impl From<LexerError> for LangError {
    fn from(error: LexerError) -> Self {
        LangError::Lexer(error)
    }
}
