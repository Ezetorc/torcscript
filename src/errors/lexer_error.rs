use std::fmt;

use crate::errors::lang_error::LangError;

#[derive(Debug)]
pub enum LexerError {
    InvalidToken(String),
}

impl From<LexerError> for LangError {
    fn from(error: LexerError) -> Self {
        LangError::Lexer(error)
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidToken(error) => write!(formatter, "{error}"),
        }
    }
}
