use std::collections::HashMap;

use crate::{
    errors::{interpreter_error::InterpreterError, lang_error::LangError},
    runtime::{native::native_property::NativeProperty, value::value::Value},
};

pub fn get_list_properties() -> HashMap<&'static str, NativeProperty> {
    let mut props: HashMap<&str, NativeProperty> = HashMap::new();

    props.insert("size", list_size);

    props
}

pub fn list_size(value: Value) -> Result<Value, LangError> {
    match value {
        Value::List(list) => Ok(Value::Number(list.borrow().len() as i64)),

        _ => Err(InterpreterError::NotFound("Method 'add' not found".to_string()).into()),
    }
}
