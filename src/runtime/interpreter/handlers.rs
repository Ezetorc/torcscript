use colored::Colorize;
use std::{cell::RefCell, rc::Rc};

use crate::{
    errors::{interpreter_error::InterpreterError, lang_error::LangError},
    runtime::{
        interpreter::interpreter::Interpreter,
        program::{expression::Expression, function::Function, statement::Statement},
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
        let function: Value = Value::Function(Function {
            parameters,
            statements,
            closure: self.environment.clone(),
        });

        self.environment
            .borrow_mut()
            .define_constant(identifier.clone(), function);

        Ok(())
    }

    pub fn handle_function_execution(
        &mut self,
        function: Function,
        arguments: Vec<Value>,
    ) -> Result<Value, LangError> {
        Interpreter::expect_parameters_amount(&function.parameters, arguments.len())?;

        self.execute_in_new_environment(function.closure.clone(), |interpreter| {
            for (parameter, argument) in function.parameters.iter().zip(arguments.into_iter()) {
                interpreter
                    .environment
                    .borrow_mut()
                    .define_constant(parameter.clone(), argument);
            }

            for statement in function.statements {
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

        if parameters.len() > 2 || parameters.len() == 0 {
            return Err(InterpreterError::InvalidAmount(format!(
                "Expected 1 or 2 parameters, got {}",
                parameters.len()
            ))
            .into());
        }

        let item_parameter_name: &String = &parameters[0];
        let index_parameter_name: Option<&String> = parameters.get(1);

        for (index, item) in items.borrow().iter().enumerate() {
            let item: Value = item.clone();

            self.execute_in_new_environment(self.environment.clone(), |interpreter| {
                interpreter
                    .environment
                    .borrow_mut()
                    .define_constant(item_parameter_name.clone(), item);

                if let Some(index_name) = index_parameter_name {
                    interpreter
                        .environment
                        .borrow_mut()
                        .define_constant(index_name.clone(), Value::Number(index as i64));
                }

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

        self.environment
            .borrow_mut()
            .define_variable(identifier, value);

        Ok(())
    }

    pub fn handle_constant_declaration(
        &mut self,
        identifier: String,
        expression: Expression,
    ) -> Result<(), LangError> {
        let value: Value = self.evaluate_expression(&expression)?;

        self.environment
            .borrow_mut()
            .define_constant(identifier, value);

        Ok(())
    }
}
