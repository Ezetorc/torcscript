use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    State,
    Action,
    Print,
    Else,
    If,
}

impl Display for Keyword {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
