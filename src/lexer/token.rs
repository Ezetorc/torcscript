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
