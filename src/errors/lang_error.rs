use std::fmt;

use crate::errors::{
    interpreter_error::InterpreterError, lexer_error::LexerError, parser_error::ParserError,
};

#[derive(Debug)]
pub enum LangError {
    Lexer(LexerError),
    Parser(ParserError),
    Interpreter(InterpreterError),
}

impl fmt::Display for LangError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LangError::Lexer(error) => write!(formatter, "Lexer error: {:?}", error),
            LangError::Parser(error) => write!(formatter, "Parser error: {:?}", error),
            LangError::Interpreter(error) => write!(formatter, "Interpreter error: {:?}", error),
        }
    }
}
