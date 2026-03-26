use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Number(i64),
    Boolean(bool),
}

impl fmt::Display for Literal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Boolean(boolean) => write!(formatter, "{}", boolean),
            Literal::String(string) => write!(formatter, "{}", string),
            Literal::Number(number) => write!(formatter, "{}", number),
        }
    }
}
