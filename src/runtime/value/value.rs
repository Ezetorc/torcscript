use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use colored::Colorize;

use crate::{
    errors::{interpreter_error::InterpreterError, lang_error::LangError},
    runtime::{native::native_method::NativeMethod, program::action::Action},
};

#[derive(Debug, Clone)]
pub enum Value {
    Number(i64),
    String(String),
    Boolean(bool),
    List(Rc<RefCell<Vec<Value>>>),
    Object(Rc<RefCell<HashMap<String, Value>>>),
    Action(Action),
    BoundMethod {
        receiver: Box<Value>,
        method: NativeMethod,
    },
    None,
}

impl Value {
    pub fn as_list(&self) -> Result<Rc<RefCell<Vec<Value>>>, LangError> {
        match self {
            Value::List(list) => Ok(list.clone()),
            _ => Err(InterpreterError::NotFound("Expected List".to_string()).into()),
        }
    }

    pub fn as_string(&self) -> Result<String, LangError> {
        match self {
            Value::String(string) => Ok(string.clone()),
            _ => Err(InterpreterError::NotFound("Expected String".to_string()).into()),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,

            (Value::List(a), Value::List(b)) => {
                let a = a.borrow();
                let b = b.borrow();
                *a == *b
            }

            (Value::Object(a), Value::Object(b)) => {
                let a = a.borrow();
                let b = b.borrow();
                *a == *b
            }

            (Value::BoundMethod { receiver: r1, .. }, Value::BoundMethod { receiver: r2, .. }) => {
                r1 == r2
            }

            (Value::None, Value::None) => true,

            _ => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(string) => write!(formatter, "'{string}'"),
            Value::Number(number) => write!(formatter, "{number}"),
            Value::Boolean(boolean) => write!(formatter, "{boolean}"),
            Value::Action(action) => write!(formatter, "{action}"),
            Value::BoundMethod {
                receiver: _,
                method: _,
            } => write!(formatter, "[BoundMethod]"),
            Value::List(list) => {
                write!(formatter, "{}", "List(".bold())?;

                for (index, value) in list.borrow().iter().enumerate() {
                    if index == list.borrow().len() - 1 {
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

                for (index, (identifier, value)) in object.borrow().iter().enumerate() {
                    if index == object.borrow().len() - 1 {
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
