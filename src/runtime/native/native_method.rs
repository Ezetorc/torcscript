use crate::{errors::lang_error::LangError, runtime::value::value::Value};

pub type NativeMethod = fn(Value, Vec<Value>) -> Result<Value, LangError>;
