use crate::abstract_syntax_tree::{literal::Literal, operator::Operator};

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Operator(Operator),
    Literal(Literal),
    EndOfLine,
    Variable,
    Equal,
}
