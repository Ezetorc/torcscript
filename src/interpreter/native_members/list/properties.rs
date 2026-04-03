use crate::{
    errors::{interpreter_error::InterpreterError, lang_error::LangError},
    interpreter::value::value::Value,
};

pub fn list_size(value: Value) -> Result<Value, LangError> {
    match value {
        Value::List(list) => Ok(Value::Number(list.borrow().len() as i64)),

        _ => Err(InterpreterError::NotFound("Method 'add' not found".to_string()).into()),
    }
}
