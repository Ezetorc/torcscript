use std::{collections::HashMap, fmt::Display};

use crate::frontend::lexer::token::{literal::Literal, operator::Operator};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(String),
    Literal(Literal),
    List(Vec<Expression>),
    Object(HashMap<String, Expression>),
    Assignment {
        target: Box<Expression>,
        expression: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
    Unary {
        operator: Operator,
        expression: Box<Expression>,
    },
    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
    Member {
        expression: Box<Expression>,
        property: String,
    },
}

impl Display for Expression {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Literal(literal) => write!(formatter, "{literal}"),
            Expression::List(list) => {
                write!(formatter, "[List]:")?;

                for expression in list {
                    write!(formatter, "{expression}")?;
                }

                Ok(())
            }
            Expression::Assignment { target, expression } => {
                write!(formatter, "[Assigment]: ({target}) = ({expression})")
            }
            Expression::Call {
                callee: _callee,
                arguments: _arguments,
            } => write!(formatter, "[Call]"),
            Expression::Member {
                expression,
                property,
            } => {
                write!(formatter, "[Member]: ({expression}).{property}")
            }
            Expression::Object(object) => {
                write!(formatter, "[Object]: ")?;

                for (index, (identifier, expression)) in object.iter().enumerate() {
                    if index == object.len() - 1 {
                        write!(formatter, "{identifier}: {expression}")?;
                    } else {
                        write!(formatter, "{identifier}: {expression}, ")?;
                    }
                }

                Ok(())
            }
            Expression::Identifier(identifier) => write!(formatter, "{identifier}"),
            Expression::Binary {
                left,
                operator,
                right,
            } => write!(formatter, "{left} {operator} {right}"),
            Expression::Unary {
                operator,
                expression,
            } => write!(formatter, "{operator}{expression}"),
        }
    }
}
