use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Function,
    Constant,
    Variable,
    Print,
    Else,
    For,
    If,
    In,
}

impl fmt::Display for Keyword {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Keyword::Variable => "State",
            Keyword::Function => "Action",
            Keyword::Print => "Print",
            Keyword::Else => "Else",
            Keyword::In => "In",
            Keyword::For => "For",
            Keyword::If => "If",
            Keyword::Constant => "Fact",
        };

        write!(formatter, "{text}")
    }
}
