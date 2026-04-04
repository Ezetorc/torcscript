use std::collections::HashMap;

use crate::{
    errors::lang_error::LangError,
    runtime::{native::native_property::NativeProperty, value::value::Value},
};

pub fn get_string_properties() -> HashMap<&'static str, NativeProperty> {
    let mut props: HashMap<&str, NativeProperty> = HashMap::new();

    props.insert("size", get_size);
    props.insert("is_empty", is_empty);

    props
}

fn get_size(value: Value) -> Result<Value, LangError> {
    let string: String = value.as_string()?;

    Ok(Value::Number(string.len() as i64))
}

fn is_empty(value: Value) -> Result<Value, LangError> {
    let string: String = value.as_string()?;

    Ok(Value::Boolean(string.len() == 0))
}
