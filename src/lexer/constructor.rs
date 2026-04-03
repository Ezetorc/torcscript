use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Constructor {
    List,
    Object,
}

impl Display for Constructor {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Constructor::List => write!(formatter, "List"),
            Constructor::Object => write!(formatter, "Object"),
        }
    }
}
