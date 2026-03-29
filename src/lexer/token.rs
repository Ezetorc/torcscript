use std::fmt::{Display, Result};

use crate::{
    abstract_syntax_tree::{literal::Literal, operator::Operator},
    lexer::{keyword::Keyword, side::Side},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Operator(Operator),
    Literal(Literal),
    Keyword(Keyword),
    Bracket(Side),
    EndOfLine,
    EndOfFile,
    Equal,
}

impl Display for Token {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result {
        write!(formatter, "{:?}", self)
    }
}
