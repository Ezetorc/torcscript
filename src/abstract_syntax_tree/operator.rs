use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Multiplication,
    Substraction,
    Difference,
    Addition,
    Division,
    Equality,
    Negation,
    Greater,
    Less,
    GreaterOrEqual,
    LessOrEqual,
}

impl Operator {
    pub fn is_binary(&self) -> bool {
        matches!(
            self,
            Operator::Addition
                | Operator::Substraction
                | Operator::Difference
                | Operator::Equality
                | Operator::Greater
                | Operator::GreaterOrEqual
                | Operator::Less
                | Operator::LessOrEqual
        )
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
            Operator::Negation => write!(formatter, "not"),
            Operator::Difference => write!(formatter, "isnt"),
            Operator::Greater => write!(formatter, ">"),
            Operator::GreaterOrEqual => write!(formatter, ">="),
            Operator::Less => write!(formatter, "<"),
            Operator::LessOrEqual => write!(formatter, "<="),
        }
    }
}
