use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Constructor {
    List,
    Object,
    String,
}

impl fmt::Display for Constructor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Constructor::List => write!(formatter, "List"),
            Constructor::Object => write!(formatter, "Object"),
            Constructor::String => write!(formatter, "String"),
        }
    }
}
