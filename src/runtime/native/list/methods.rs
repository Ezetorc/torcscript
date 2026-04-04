use std::collections::HashMap;

use crate::{
    errors::{interpreter_error::InterpreterError, lang_error::LangError},
    runtime::{native::native_method::NativeMethod, value::value::Value},
};

pub fn get_list_methods() -> HashMap<&'static str, NativeMethod> {
    let mut methods: HashMap<&str, NativeMethod> = HashMap::new();

    methods.insert("add_last", add_element_at_end);
    methods.insert("add_first", add_element_at_start);
    methods.insert("remove", remove_element);
    methods.insert("remove_first", remove_element_at_start);
    methods.insert("remove_last", remove_element_at_end);
    methods.insert("clear", clear);
    methods.insert("get", get_element);
    methods.insert("get_at", get_element_at);
    methods.insert("has", has_element);
    methods.insert("get_index_of", get_index_of);

    methods
}

fn add_element_at_end(value: Value, arguments: Vec<Value>) -> Result<Value, LangError> {
    let list_rc = value.as_list()?;

    for argument in arguments {
        list_rc.borrow_mut().push(argument);
    }

    Ok(Value::List(list_rc))
}

fn add_element_at_start(value: Value, arguments: Vec<Value>) -> Result<Value, LangError> {
    let list_rc = value.as_list()?;

    for argument in arguments.into_iter().rev() {
        list_rc.borrow_mut().insert(0, argument);
    }

    Ok(Value::List(list_rc))
}

fn remove_element(value: Value, arguments: Vec<Value>) -> Result<Value, LangError> {
    let list_rc = value.as_list()?;

    {
        let mut list = list_rc.borrow_mut();
        list.retain(|value| !arguments.contains(value));
    }

    Ok(Value::List(list_rc))
}

fn remove_element_at_start(value: Value, _arguments: Vec<Value>) -> Result<Value, LangError> {
    let list_rc = value.as_list()?;

    {
        let mut list = list_rc.borrow_mut();

        if !list.is_empty() {
            list.remove(0);
        }
    }

    Ok(Value::List(list_rc))
}

fn remove_element_at_end(value: Value, _arguments: Vec<Value>) -> Result<Value, LangError> {
    let list_rc = value.as_list()?;

    {
        let mut list = list_rc.borrow_mut();
        list.pop();
    }

    Ok(Value::List(list_rc))
}

fn get_element(value: Value, arguments: Vec<Value>) -> Result<Value, LangError> {
    let list_rc = value.as_list()?;
    let list = list_rc.borrow();

    let target = match arguments.get(0) {
        Some(value) => value,
        None => return Ok(Value::None),
    };

    for value in list.iter() {
        if value == target {
            return Ok(value.clone());
        }
    }

    Ok(Value::None)
}

fn get_element_at(value: Value, arguments: Vec<Value>) -> Result<Value, LangError> {
    let list_rc = value.as_list()?;
    let list = list_rc.borrow();

    let target: &Value = match arguments.get(0) {
        Some(value) => value,
        None => return Ok(Value::None),
    };

    match target {
        Value::Number(index) => {
            let index: usize = *index as usize;

            match list.get(index) {
                Some(value) => Ok(value.clone()),
                None => Ok(Value::None),
            }
        }
        _ => Err(InterpreterError::NotFound("Expected index".to_string()).into()),
    }
}

fn has_element(value: Value, arguments: Vec<Value>) -> Result<Value, LangError> {
    let list_rc = value.as_list()?;
    let list = list_rc.borrow();

    let target: &Value = match arguments.get(0) {
        Some(value) => value,
        None => return Ok(Value::None),
    };

    for value in list.iter() {
        if value == target {
            return Ok(Value::Boolean(true));
        }
    }

    Ok(Value::None)
}

fn get_index_of(value: Value, arguments: Vec<Value>) -> Result<Value, LangError> {
    let list_rc = value.as_list()?;
    let list = list_rc.borrow();

    let target: &Value = match arguments.get(0) {
        Some(value) => value,
        None => return Ok(Value::None),
    };

    for (index, value) in list.iter().enumerate() {
        if value == target {
            return Ok(Value::Number(index as i64));
        }
    }

    Ok(Value::None)
}

fn clear(value: Value, _arguments: Vec<Value>) -> Result<Value, LangError> {
    let list_rc = value.as_list()?;

    {
        let mut list = list_rc.borrow_mut();
        list.clear();
    }

    Ok(Value::List(list_rc))
}
