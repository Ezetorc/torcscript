use crate::errors::lang_error::LangError;

#[derive(Debug)]
pub enum ParserError {
    NotImplemented(String),
    InvalidSyntax(String),
}

impl From<ParserError> for LangError {
    fn from(error: ParserError) -> Self {
        LangError::Parser(error)
    }
}
