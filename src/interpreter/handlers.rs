use crate::{
    abstract_syntax_tree::expression::Expression,
    errors::{interpreter_error::InterpreterError, lang_error::LangError},
    interpreter::interpreter::Interpreter,
};

impl Interpreter {
    pub fn handle_print(&mut self, expression: Expression) -> Result<(), LangError> {
        match expression {
            Expression::Literal(literal) => println!("{}", literal.to_string()),
            _ => {
                return Err(LangError::Interpreter(InterpreterError::NotImplemented(
                    "Expression print not yet implemented".to_string(),
                )));
            }
        }

        Ok(())
    }
}
