use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Addition,
    Substraction,
    Multiplication,
    Division,
}

impl Display for Operator {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Multiplication => write!(formatter, "*"),
            Operator::Substraction => write!(formatter, "-"),
            Operator::Addition => write!(formatter, "+"),
            Operator::Division => write!(formatter, "/"),
        }
    }
}
