use crate::{abstract_syntax_tree::statement::Statement, errors::lang_error::LangError};

pub struct Interpreter {}

impl Interpreter {
    fn new() -> Self {
        Self {}
    }

    pub fn execute(statements: Vec<Statement>) -> Result<(), LangError> {
        let mut interpreter: Interpreter = Interpreter::new();

        for statement in statements {
            match statement {
                Statement::Print { expression } => interpreter.handle_print(expression)?,
            }
        }

        Ok(())
    }
}
