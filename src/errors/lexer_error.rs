use std::fmt::Display;

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

impl Display for LexerError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidToken(error) => write!(formatter, "{error}"),
        }
    }
}
