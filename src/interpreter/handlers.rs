use colored::Colorize;
use std::{cell::RefCell, rc::Rc};

use crate::{
    abstract_syntax_tree::{action::Action, expression::Expression, statement::Statement},
    errors::{interpreter_error::InterpreterError, lang_error::LangError},
    interpreter::{environment::Environment, interpreter::Interpreter, value::value::Value},
};

impl Interpreter {
    pub fn handle_print(&mut self, expression: &Expression) -> Result<(), LangError> {
        let value: Value = self.evaluate_expression(expression)?;

        println!("{} {value}", ">".green());

        Ok(())
    }

    pub fn handle_action_declaration(
        &mut self,
        identifier: String,
        parameters: Vec<String>,
        statements: Vec<Statement>,
    ) -> Result<(), LangError> {
        let action: Value = Value::Action(Action {
            parameters,
            statements,
            closure: self.environment.clone(),
        });

        self.environment
            .borrow_mut()
            .set(identifier.clone(), action);

        Ok(())
    }

    pub fn handle_action_execution(
        &mut self,
        identifier: String,
        arguments: Vec<Expression>,
    ) -> Result<(), LangError> {
        let action: Option<Value> = self.environment.borrow().get(&identifier);
        let arguments: Vec<Value> = arguments
            .into_iter()
            .map(|arg| self.evaluate_expression(&arg))
            .collect::<Result<Vec<_>, _>>()?;

        let action: Action = match action {
            Some(Value::Action(action)) => action,
            Some(_) => {
                return Err(InterpreterError::NotFound(format!(
                    "'{identifier}' is not a function"
                ))
                .into());
            }
            None => {
                return Err(
                    InterpreterError::NotFound(format!("Action '{identifier}' not found")).into(),
                );
            }
        };

        if action.parameters.len() != arguments.len() {
            return Err(InterpreterError::InvalidAmount(format!(
                "Expected {} arguments, got {}",
                action.parameters.len(),
                arguments.len()
            ))
            .into());
        }

        let new_environment: Rc<RefCell<Environment>> = Rc::new(RefCell::new(
            Environment::new_with_parent(action.closure.clone()),
        ));

        for (parameter, argument) in action.parameters.iter().zip(arguments.into_iter()) {
            new_environment
                .borrow_mut()
                .set(parameter.clone(), argument);
        }

        let previous_environment: Rc<RefCell<Environment>> = self.environment.clone();
        self.environment = new_environment;

        for statement in action.statements {
            self.execute_statement(statement)?;
        }

        self.environment = previous_environment;

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
            Value::None => false,
            Value::Boolean(boolean) => boolean,
            Value::String(string) => !string.is_empty(),
            Value::Number(number) => number > 0,
            Value::List(list) => list.len() > 0,
            Value::Object(object) => object.len() > 0,
            Value::Action(_) => true,
        };

        if condition_satisfied {
            self.execute_block(statements)?;
        } else if let Some(else_statements) = else_statements {
            self.execute_block(else_statements)?;
        }

        Ok(())
    }

    pub fn handle_state_declaration(
        &mut self,
        identifier: String,
        expression: Expression,
    ) -> Result<(), LangError> {
        let value: Value = self.evaluate_expression(&expression)?;

        self.environment.borrow_mut().set(identifier, value);

        Ok(())
    }

    pub fn handle_state_assignation(
        &mut self,
        identifier: String,
        expression: Expression,
    ) -> Result<(), LangError> {
        let current_value: Option<Value> = self.environment.borrow().get(identifier.as_str());

        if let Some(_) = current_value {
            let new_value: Value = self.evaluate_expression(&expression)?;

            self.environment.borrow_mut().set(identifier, new_value);

            Ok(())
        } else {
            return Err(
                InterpreterError::NotFound(format!("State '{identifier}' not found")).into(),
            );
        }
    }
}
