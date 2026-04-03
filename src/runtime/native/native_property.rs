use crate::{errors::lang_error::LangError, runtime::value::value::Value};

pub type NativeProperty = fn(Value) -> Result<Value, LangError>;
