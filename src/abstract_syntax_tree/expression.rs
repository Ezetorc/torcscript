use std::fmt::Display;

use crate::{
    abstract_syntax_tree::{literal::Literal, operator::Operator},
    errors::lang_error::LangError,
    errors::parser_error::ParserError,
    lexer::token::Token,
};

#[derive(Debug)]
pub enum Expression {
    Identifier(String),
    Literal(Literal),
    Binary {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
}

impl Display for Expression {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Literal(literal) => match literal {
                Literal::String(string) => write!(formatter, "{}", string),
                Literal::Number(number) => write!(formatter, "{}", number),
                Literal::Boolean(boolean) => write!(formatter, "{}", boolean),
            },
            Expression::Identifier(identifier) => write!(formatter, "{}", identifier),
            Expression::Binary {
                left,
                operator,
                right,
            } => write!(formatter, "({} {} {})", operator, left, right),
        }
    }
}

impl TryFrom<Token> for Expression {
    type Error = LangError;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Literal(literal) => Ok(Expression::Literal(literal)),
            _ => {
                return Err(LangError::Parser(ParserError::NotImplemented(
                    "Printable token recognitizion not yet implemented".to_string(),
                )));
            }
        }
    }
}
