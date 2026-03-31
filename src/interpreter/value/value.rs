use std::fmt::{Display, Formatter, Result};

use crate::abstract_syntax_tree::action::Action;

#[derive(Debug, Clone)]
pub enum Value {
    Number(i64),
    String(String),
    Boolean(bool),
    List(Vec<Value>),
    Action(Action),
    None,
}

impl Display for Value {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Value::String(string) => write!(formatter, "'{string}'"),
            Value::Number(number) => write!(formatter, "{number}"),
            Value::Boolean(boolean) => write!(formatter, "{boolean}"),
            Value::Action(action) => write!(formatter, "{action}"),
            Value::List(list) => {
                write!(formatter, "List(")?;

                for (index, value) in list.iter().enumerate() {
                    if index == list.len() - 1 {
                        write!(formatter, "{value}")?;
                    } else {
                        write!(formatter, "{value}, ")?;
                    }
                }

                write!(formatter, ")")?;

                Ok(())
            }
            Value::None => write!(formatter, "None"),
        }
    }
}
