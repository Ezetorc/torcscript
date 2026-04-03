use crate::{errors::lang_error::LangError, interpreter::value::value::Value};

pub type NativeProperty = fn(Value) -> Result<Value, LangError>;
