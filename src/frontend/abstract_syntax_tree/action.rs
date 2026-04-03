use std::{cell::RefCell, fmt, rc::Rc};

use crate::{
    frontend::abstract_syntax_tree::statement::Statement, runtime::environment::Environment,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Action {
    pub parameters: Vec<String>,
    pub statements: Vec<Statement>,
    pub closure: Rc<RefCell<Environment>>,
}

impl fmt::Display for Action {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Action({:?})", self.parameters)
    }
}
