use std::collections::HashMap;

use crate::{
    errors::{interpreter_error::InterpreterError, lang_error::LangError},
    runtime::{native::native_method::NativeMethod, value::value::Value},
};

pub fn get_string_methods() -> HashMap<&'static str, NativeMethod> {
    let mut methods: HashMap<&str, NativeMethod> = HashMap::new();

    methods.insert("in_uppercase", in_uppercase);
    methods.insert("in_lowercase", in_lowercase);
    methods.insert("capitalize", capitalize);
    methods.insert("replace", replace);
    methods.insert("trim", trim);
    methods.insert("has", has);
    methods.insert("starts_with", starts_with);
    methods.insert("ends_with", ends_with);

    methods
}

fn in_uppercase(value: Value, _arguments: Vec<Value>) -> Result<Value, LangError> {
    let string = value.as_string()?;
    Ok(Value::String(string.to_uppercase()))
}

fn in_lowercase(value: Value, _arguments: Vec<Value>) -> Result<Value, LangError> {
    let string = value.as_string()?;
    Ok(Value::String(string.to_lowercase()))
}

fn capitalize(value: Value, _arguments: Vec<Value>) -> Result<Value, LangError> {
    let string: String = value.as_string()?;

    let mut chars = string.chars();

    let result: String = match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    };

    Ok(Value::String(result))
}

fn replace(value: Value, arguments: Vec<Value>) -> Result<Value, LangError> {
    if arguments.len() != 2 {
        return Err(InterpreterError::InvalidAmount(format!(
            "Expected 2 arguments, found {}",
            arguments.len()
        ))
        .into());
    }

    let string: String = value.as_string()?;
    let from = arguments[0].as_string()?;
    let to = arguments[1].as_string()?;

    Ok(Value::String(string.replace(&from, &to)))
}

fn trim(value: Value, _args: Vec<Value>) -> Result<Value, LangError> {
    let string: String = value.as_string()?;
    Ok(Value::String(string.trim().to_string()))
}

fn has(value: Value, arguments: Vec<Value>) -> Result<Value, LangError> {
    if arguments.len() != 1 {
        return Err(InterpreterError::InvalidAmount(format!(
            "Expected 1 arguments, found {}",
            arguments.len()
        ))
        .into());
    }

    let string: String = value.as_string()?;
    let needle: String = arguments[0].as_string()?;

    Ok(Value::Boolean(string.contains(&needle)))
}

fn starts_with(value: Value, arguments: Vec<Value>) -> Result<Value, LangError> {
    if arguments.len() != 1 {
        return Err(InterpreterError::InvalidAmount(format!(
            "Expected 1 arguments, found {}",
            arguments.len()
        ))
        .into());
    }

    let string: String = value.as_string()?;
    let prefix: String = arguments[0].as_string()?;

    Ok(Value::Boolean(string.starts_with(&prefix)))
}

fn ends_with(value: Value, arguments: Vec<Value>) -> Result<Value, LangError> {
    if arguments.len() != 1 {
        return Err(InterpreterError::InvalidAmount(format!(
            "Expected 1 arguments, found {}",
            arguments.len()
        ))
        .into());
    }

    let string: String = value.as_string()?;
    let suffix: String = arguments[0].as_string()?;

    Ok(Value::Boolean(string.ends_with(&suffix)))
}
