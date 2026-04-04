use std::fmt;

use crate::errors::lang_error::LangError;

#[derive(Debug)]
pub enum ParserError {
    InvalidSyntax(String),
    NotFound(String),
}

impl From<ParserError> for LangError {
    fn from(error: ParserError) -> Self {
        LangError::Parser(error)
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSyntax(error) | Self::NotFound(error) => {
                write!(formatter, "{error}")
            }
        }
    }
}
