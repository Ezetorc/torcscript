use crate::runtime::value::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct EnvironmentValue {
    pub value: Value,
    pub is_mutable: bool,
}

impl EnvironmentValue {
    pub fn mutable(value: Value) -> Self {
        Self {
            value,
            is_mutable: true,
        }
    }

    pub fn immutable(value: Value) -> Self {
        Self {
            value,
            is_mutable: false,
        }
    }
}
