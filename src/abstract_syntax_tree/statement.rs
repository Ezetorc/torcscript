use colored::Colorize;
use std::fmt::{Display, Result};

use crate::abstract_syntax_tree::expression::Expression;

#[derive(Debug, Clone)]
pub enum Statement {
    Print {
        expression: Expression,
    },
    StateDeclaration {
        identifier: String,
        expression: Expression,
    },
    StateAssignation {
        identifier: String,
        expression: Expression,
    },
    ActionDeclaration {
        identifier: String,
        parameters: Vec<String>,
        statements: Vec<Statement>,
    },
    ActionExecution {
        identifier: String,
        parameters: Vec<Expression>,
    },
    Conditional {
        condition: Expression,
        statements: Vec<Statement>,
        else_statements: Option<Vec<Statement>>,
    },
}

impl Display for Statement {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result {
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
            Statement::StateAssignation {
                identifier,
                expression,
            } => write!(
                formatter,
                "{}{}, {}{}",
                "StateAssignation(".blue(),
                identifier.italic(),
                expression.to_string().italic(),
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
            Statement::ActionExecution {
                identifier,
                parameters,
            } => {
                write!(formatter, "{}", "ActionExecution(".blue())?;
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
        }
    }
}
