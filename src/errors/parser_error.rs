use std::fmt::Display;

use crate::errors::lang_error::LangError;

#[derive(Debug)]
pub enum ParserError {
    NotImplemented(String),
    InvalidSyntax(String),
    NotFound(String),
}

impl From<ParserError> for LangError {
    fn from(error: ParserError) -> Self {
        LangError::Parser(error)
    }
}

impl Display for ParserError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotImplemented(error) | Self::InvalidSyntax(error) | Self::NotFound(error) => {
                write!(formatter, "{error}")
            }
        }
    }
}
