use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Action,
    State,
    Print,
    Else,
    For,
    If,
    In,
}

impl fmt::Display for Keyword {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Keyword::State => "State",
            Keyword::Action => "Action",
            Keyword::Print => "Print",
            Keyword::Else => "Else",
            Keyword::In => "In",
            Keyword::For => "For",
            Keyword::If => "If",
        };

        write!(formatter, "{text}")
    }
}
