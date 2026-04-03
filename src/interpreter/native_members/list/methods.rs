use crate::{
    errors::{interpreter_error::InterpreterError, lang_error::LangError},
    interpreter::value::value::Value,
};

pub fn list_add_element(list: Value, arguments: Vec<Value>) -> Result<Value, LangError> {
    match list {
        Value::List(list) => {
            for argument in arguments {
                list.borrow_mut().push(argument);
            }

            Ok(Value::List(list))
        }
        _ => Err(InterpreterError::NotFound(format!("Method not found")).into()),
    }
}

pub fn list_remove_element(list: Value, arguments: Vec<Value>) -> Result<Value, LangError> {
    match list {
        Value::List(list_rc) => {
            {
                let mut list = list_rc.borrow_mut();
                list.retain(|value| !arguments.contains(value));
            }

            Ok(Value::List(list_rc))
        }
        _ => Err(InterpreterError::NotFound("Method not found".to_string()).into()),
    }
}

pub fn list_get_element(list: Value, arguments: Vec<Value>) -> Result<Value, LangError> {
    match list {
        Value::List(list_rc) => {
            let list: std::cell::Ref<'_, Vec<Value>> = list_rc.borrow();

            let target: &Value = match arguments.get(0) {
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
        _ => Err(InterpreterError::NotFound("Method not found".to_string()).into()),
    }
}

pub fn list_get_element_at(list: Value, arguments: Vec<Value>) -> Result<Value, LangError> {
    match list {
        Value::List(list_rc) => {
            let list = list_rc.borrow();

            let target = match arguments.get(0) {
                Some(value) => value,
                None => return Ok(Value::None),
            };

            match target {
                Value::Number(index) => {
                    let index = *index as usize;

                    match list.get(index) {
                        Some(value) => Ok(value.clone()),
                        None => Ok(Value::None),
                    }
                }
                _ => Err(InterpreterError::NotFound("Expected index".to_string()).into()),
            }
        }
        _ => Err(InterpreterError::NotFound("Method not found".to_string()).into()),
    }
}
