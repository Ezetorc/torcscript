use crate::frontend::literal::literal::Literal;

#[derive(Debug)]
pub enum Token {
    Variable,
    Identifier(String),
    Equal,
    Literal(Literal),
    Semicolon,
}
