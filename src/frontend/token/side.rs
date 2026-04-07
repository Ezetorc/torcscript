use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum Side {
    Left,
    Right,
}

impl fmt::Display for Side {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Side::Left => "Left",
            Side::Right => "Right",
        };

        write!(formatter, "{text}")
    }
}
