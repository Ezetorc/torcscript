use crate::{
    abstract_syntax_tree::expression::Expression,
    errors::lang_error::LangError,
    interpreter::{interpreter::Interpreter, value::Value},
};

impl Interpreter {
    pub fn handle_print(&mut self, expression: &Expression) -> Result<(), LangError> {
        let value: Value = self.evaluate_expression(expression)?;

        match value {
            Value::Number(number) => println!("{}", number),
            Value::String(string) => println!("{}", string),
            Value::Boolean(boolean) => println!("{}", boolean),
        }

        Ok(())
    }

    pub fn handle_variable_declaration(
        &mut self,
        identifier: String,
        expression: Expression,
    ) -> Result<(), LangError> {
        let value: Value = self.evaluate_expression(&expression)?;

        self.environment.set_variable(identifier, value);

        Ok(())
    }
}
