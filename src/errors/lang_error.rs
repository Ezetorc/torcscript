use std::fmt;

use colored::Colorize;

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
            LangError::Lexer(error) => {
                write!(formatter, "{} {error}", "[Lexer Error]".red().bold())
            }
            LangError::Parser(error) => {
                write!(formatter, "{} {error}", "[Parser Error]".red().bold())
            }
            LangError::Interpreter(error) => {
                write!(formatter, "{} {error}", "[Interpreter Error]".red().bold())
            }
        }
    }
}
