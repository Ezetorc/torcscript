use crate::abstract_syntax_tree::expression::Expression;

#[derive(Debug)]
pub enum Statement {
    Print {
        expression: Expression,
    },
    Expression {
        expression: Expression,
    },
    VariableDeclaration {
        identifier: String,
        expression: Expression,
    },
    VariableAssignation {
        identifier: String,
        expression: Expression,
    },
    Conditional {
        condition: Expression,
        statements: Vec<Statement>,
    },
}
