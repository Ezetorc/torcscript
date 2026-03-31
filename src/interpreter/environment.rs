use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::interpreter::value::value::Value;

#[derive(Debug, Clone)]
pub struct Environment {
    pub values: HashMap<String, Value>,
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

    pub fn set(&mut self, name: String, value: Value) -> () {
        if let Some(parent) = &self.parent {
            return parent.borrow_mut().set(name, value);
        } else {
            self.values.insert(name, value);
        }
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.values.get(name) {
            return Some(value.clone());
        }

        if let Some(parent) = &self.parent {
            return parent.borrow().get(name);
        }

        None
    }
}
