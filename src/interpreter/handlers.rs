use crate::{
    abstract_syntax_tree::{expression::Expression, statement::Statement},
    errors::{interpreter_error::InterpreterError, lang_error::LangError},
    interpreter::{interpreter::Interpreter, value::Value},
};

impl Interpreter {
    pub fn handle_print(&mut self, expression: &Expression) -> Result<(), LangError> {
        let value: Value = self.evaluate_expression(expression)?;

        match value {
            Value::Number(number) => println!("{}", number),
            Value::String(string) => println!("{}", string),
            Value::Boolean(boolean) => println!("{}", boolean),
            Value::None => println!("None"),
        }

        Ok(())
    }

    pub fn handle_conditional_statements(
        &mut self,
        condition: Expression,
        statements: Vec<Statement>,
        else_statements: Option<Vec<Statement>>,
    ) -> Result<(), LangError> {
        let value: Value = self.evaluate_expression(&condition)?;

        let condition_satisfied: bool = match value {
            Value::None => return Ok(()),
            Value::Boolean(boolean) => boolean,
            Value::String(string) => !string.is_empty(),
            Value::Number(number) => number > 0,
        };

        if condition_satisfied {
            self.execute_block(statements)?;
        } else if let Some(else_statements) = else_statements {
            self.execute_block(else_statements)?;
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

    pub fn handle_variable_assignation(
        &mut self,
        identifier: String,
        expression: Expression,
    ) -> Result<(), LangError> {
        let current_value: Option<&Value> = self.environment.get_variable(identifier.as_str());

        if let Some(_) = current_value {
            let new_value: Value = self.evaluate_expression(&expression)?;

            self.environment.set_variable(identifier, new_value);

            Ok(())
        } else {
            return Err(
                InterpreterError::NotFound(format!("Variable '{identifier}' not found")).into(),
            );
        }
    }
}
