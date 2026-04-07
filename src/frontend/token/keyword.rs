use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    PrintNamed,
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
            Keyword::PrintNamed => "Print Named",
            Keyword::Function => "Action",
            Keyword::Variable => "State",
            Keyword::Constant => "Fact",
            Keyword::Print => "Print",
            Keyword::Else => "Else",
            Keyword::For => "For",
            Keyword::In => "In",
            Keyword::If => "If",
        };

        write!(formatter, "{text}")
    }
}
