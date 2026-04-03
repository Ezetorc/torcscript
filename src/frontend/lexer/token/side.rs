use std::fmt::Display;

#[derive(PartialEq, Debug, Clone)]
pub enum Side {
    Left,
    Right,
}

impl Display for Side {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Side::Left => "Left",
            Side::Right => "Right",
        };

        write!(formatter, "{text}")
    }
}
