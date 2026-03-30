use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum Value {
    Number(i64),
    String(String),
    Boolean(bool),
    List(Vec<Value>),
    None,
}

impl Display for Value {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Value::String(string) => write!(formatter, "[String {string}]"),
            Value::Number(number) => write!(formatter, "[Number {number}]"),
            Value::Boolean(boolean) => write!(formatter, "[Boolean {boolean}]"),
            Value::List(list) => {
                write!(formatter, "[List]:")?;

                for value in list {
                    write!(formatter, "{value}")?;
                }

                Ok(())
            }
            Value::None => write!(formatter, "None"),
        }
    }
}
