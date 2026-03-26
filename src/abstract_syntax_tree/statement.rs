use crate::abstract_syntax_tree::expression::Expression;

#[derive(Debug)]
pub enum Statement {
    Print { expression: Expression },
}
