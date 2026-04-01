use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result},
};

use colored::Colorize;

use crate::abstract_syntax_tree::action::Action;

#[derive(Debug, Clone)]
pub enum Value {
    Number(i64),
    String(String),
    Boolean(bool),
    List(Vec<Value>),
    Object(HashMap<String, Value>),
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
                write!(formatter, "{}", "List(".bold())?;

                for (index, value) in list.iter().enumerate() {
                    if index == list.len() - 1 {
                        write!(formatter, "{}", value.to_string().italic())?;
                    } else {
                        write!(formatter, "{}, ", value.to_string().italic())?;
                    }
                }

                write!(formatter, "{}", ")".bold())?;

                Ok(())
            }
            Value::Object(object) => {
                write!(formatter, "{}", "Object(".bold())?;

                for (index, (identifier, value)) in object.iter().enumerate() {
                    if index == object.len() - 1 {
                        write!(formatter, "{identifier}: {}", value.to_string().italic())?;
                    } else {
                        write!(formatter, "{identifier}: {}, ", value.to_string().italic())?;
                    }
                }

                write!(formatter, "{}", ")".bold())?;

                Ok(())
            }
            Value::None => write!(formatter, "None"),
        }
    }
}
