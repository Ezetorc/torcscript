use crate::{
    abstract_syntax_tree::{literal::Literal, operator::Operator},
    lexer::keyword::Keyword,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Operator(Operator),
    Literal(Literal),
    EndOfLine,
    EndOfFile,
    Equal,
    Keyword(Keyword),
}
