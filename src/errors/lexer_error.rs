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
