use colored::Colorize;
use std::fmt::{self};

use crate::runtime::program::expression::Expression;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Print {
        expression: Expression,
    },
    StateDeclaration {
        identifier: String,
        expression: Expression,
    },
    ActionDeclaration {
        identifier: String,
        parameters: Vec<String>,
        statements: Vec<Statement>,
    },
    Conditional {
        condition: Expression,
        statements: Vec<Statement>,
        else_statements: Option<Vec<Statement>>,
    },
    ForLoop {
        iterator: Expression,
        parameters: Vec<String>,
        statements: Vec<Statement>,
    },
    Expression {
        expression: Expression,
    },
}

impl fmt::Display for Statement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Print { expression } => write!(
                formatter,
                "{}{}{}",
                "Print(".blue(),
                expression.to_string().italic(),
                ")".blue()
            ),

            Statement::StateDeclaration {
                identifier,
                expression,
            } => write!(
                formatter,
                "{}{}, {}{}",
                "StateDeclaration(".blue(),
                identifier.italic(),
                expression.to_string().italic(),
                ")".blue()
            ),
            Statement::ForLoop {
                iterator,
                statements: _,
                parameters: _,
            } => write!(
                formatter,
                "{}{}{}",
                "ForLoop(".blue(),
                iterator.to_string().italic(),
                ")".blue()
            ),
            Statement::ActionDeclaration {
                identifier,
                parameters,
                statements: _statements,
            } => {
                write!(formatter, "{}", "ActionDeclaration(".blue())?;
                write!(formatter, "{}, ", identifier.italic())?;

                write!(
                    formatter,
                    "[{}]",
                    parameters
                        .iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )?;

                write!(formatter, "{}", ")".blue())
            }
            Statement::Conditional {
                condition,
                statements: _statements,
                else_statements: _else_statements,
            } => write!(
                formatter,
                "{}{}{}",
                "Conditional(".blue(),
                condition.to_string().italic(),
                ")".blue()
            ),
            Statement::Expression { expression } => write!(
                formatter,
                "{}{}{}",
                "Expression(".blue(),
                expression.to_string().italic(),
                ")".blue()
            ),
        }
    }
}
