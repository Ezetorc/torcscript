use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    errors::{interpreter_error::InterpreterError, lang_error::LangError},
    runtime::{environment::environment_value::EnvironmentValue, value::value::Value},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    pub values: HashMap<String, EnvironmentValue>,
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            parent: None,
        }
    }

    pub fn new_with_parent(parent: Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn define_variable(&mut self, name: String, value: Value) {
        self.values.insert(name, EnvironmentValue::mutable(value));
    }

    pub fn define_constant(&mut self, name: String, value: Value) {
        self.values.insert(name, EnvironmentValue::immutable(value));
    }

    pub fn assign(&mut self, name: String, value: Value) -> Result<(), LangError> {
        if self.values.contains_key(&name) {
            self.values.insert(name, EnvironmentValue::mutable(value));
            return Ok(());
        }

        if let Some(parent) = &self.parent {
            return parent.borrow_mut().assign(name, value);
        }

        Err(InterpreterError::NotFound(format!("State '{name}' not found")).into())
    }

    pub fn get(&self, name: &str) -> Option<EnvironmentValue> {
        if let Some(environment_value) = self.values.get(name) {
            return Some(environment_value.clone());
        }

        if let Some(parent) = &self.parent {
            return parent.borrow().get(name);
        }

        None
    }
}
