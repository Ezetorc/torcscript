use colored::Colorize;
use std::{cell::RefCell, rc::Rc};

use crate::{
    errors::lang_error::LangError,
    runtime::{
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
        Interpreter::expect_parameters_amount(&action.parameters, arguments.len())?;

        self.execute_in_new_environment(action.closure.clone(), |interpreter| {
            for (parameter, argument) in action.parameters.iter().zip(arguments.into_iter()) {
                interpreter
                    .environment
                    .borrow_mut()
                    .define(parameter.clone(), argument);
            }

            for statement in action.statements {
                interpreter.execute_statement(statement)?;
            }

            Ok(())
        })?;

        Ok(Value::None)
    }

    pub fn handle_for_loop(
        &mut self,
        iterator: Expression,
        parameters: Vec<String>,
        statements: Vec<Statement>,
    ) -> Result<(), LangError> {
        let value: Value = self.evaluate_expression(&iterator)?;
        let items: Rc<RefCell<Vec<Value>>> = value.as_iterable()?;

        Interpreter::expect_parameters_amount(&parameters, 1)?;

        let parameter_name: &String = &parameters[0];

        for item in items.borrow().iter() {
            let item: Value = item.clone();

            self.execute_in_new_environment(self.environment.clone(), |interpreter| {
                interpreter
                    .environment
                    .borrow_mut()
                    .define(parameter_name.clone(), item);

                for statement in &statements {
                    interpreter.execute_statement(statement.clone())?;
                }

                Ok(())
            })?;
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
