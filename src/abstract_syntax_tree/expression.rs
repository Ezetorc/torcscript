use std::{collections::HashMap, fmt::Display};

use crate::abstract_syntax_tree::{literal::Literal, operator::Operator};

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(String),
    Literal(Literal),
    List(Vec<Expression>),
    Object(HashMap<String, Expression>),
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
            Expression::List(list) => {
                write!(formatter, "[List]:")?;

                for expression in list {
                    write!(formatter, "{expression}")?;
                }

                Ok(())
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
