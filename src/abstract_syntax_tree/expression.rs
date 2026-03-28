use std::fmt::Display;

use crate::abstract_syntax_tree::{literal::Literal, operator::Operator};

#[derive(Debug)]
pub enum Expression {
    Identifier(String),
    Literal(Literal),
    Binary {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
    Unary {
        operator: Operator,
        expression: Box<Expression>,
    },
}

impl Display for Expression {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Literal(literal) => write!(formatter, "{literal}"),
            Expression::Identifier(identifier) => write!(formatter, "{}", identifier),
            Expression::Binary {
                left,
                operator,
                right,
            } => write!(formatter, "({} {} {})", operator, left, right),
            Expression::Unary {
                operator,
                expression,
            } => write!(formatter, "({} {})", operator, expression),
        }
    }
}
