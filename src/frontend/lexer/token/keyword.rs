use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    State,
    Action,
    Print,
    Else,
    If,
}

impl fmt::Display for Keyword {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Keyword::State => "State",
            Keyword::Action => "Action",
            Keyword::Print => "Print",
            Keyword::Else => "Else",
            Keyword::If => "If",
        };

        write!(formatter, "{text}")
    }
}
