use colored::Colorize;
use std::{cell::RefCell, rc::Rc};

use crate::{
    errors::{interpreter_error::InterpreterError, lang_error::LangError},
    runtime::{
        environment::Environment,
        interpreter::interpreter::Interpreter,
        program::{action::Action, expression::Expression, statement::Statement},
        value::value::Value,
    },
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
            .define(identifier.clone(), action);

        Ok(())
    }

    pub fn handle_action_execution(
        &mut self,
        action: Action,
        arguments: Vec<Value>,
    ) -> Result<Value, LangError> {
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
                .define(parameter.clone(), argument);
        }

        let previous_environment = self.environment.clone();

        self.environment = new_environment;

        for statement in action.statements {
            self.execute_statement(statement)?;
        }

        self.environment = previous_environment;

        Ok(Value::None)
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
            Value::List(list) => list.borrow().len() > 0,
            Value::Object(object) => object.borrow().len() > 0,
            _ => true,
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

        self.environment.borrow_mut().define(identifier, value);

        Ok(())
    }
}
