use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Addition,
    Substraction,
    Multiplication,
    Division,
    Equality,
    Negation,
}

impl Operator {
    pub fn is_additive(&self) -> bool {
        matches!(self, Operator::Addition | Operator::Substraction)
    }

    pub fn is_multiplicative(&self) -> bool {
        matches!(self, Operator::Multiplication | Operator::Division)
    }
}

impl Display for Operator {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Operator::Multiplication => write!(formatter, "*"),
            Operator::Substraction => write!(formatter, "-"),
            Operator::Addition => write!(formatter, "+"),
            Operator::Division => write!(formatter, "/"),
            Operator::Equality => write!(formatter, "is"),
            Operator::Negation => write!(formatter, "!"),
        }
    }
}
