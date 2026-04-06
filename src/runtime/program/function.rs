use std::{cell::RefCell, fmt, rc::Rc};

use crate::runtime::{environment::environment::Environment, program::statement::Statement};

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub parameters: Vec<String>,
    pub statements: Vec<Statement>,
    pub closure: Rc<RefCell<Environment>>,
}

impl fmt::Display for Function {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Action({:?})", self.parameters)
    }
}
