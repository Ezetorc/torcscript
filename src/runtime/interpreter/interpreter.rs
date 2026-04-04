use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    errors::{interpreter_error::InterpreterError, lang_error::LangError},
    frontend::lexer::token::literal::Literal,
    runtime::{
        environment::Environment,
        native::{
            list::{methods::get_list_methods, properties::get_list_properties},
            native_method::NativeMethod,
            string::{methods::get_string_methods, properties::get_string_properties},
        },
        program::{expression::Expression, program::Program, statement::Statement},
        value::value::Value,
    },
};

pub struct Interpreter {
    pub environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    fn new() -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn execute(program: Program) -> Result<(), LangError> {
        let mut interpreter: Interpreter = Interpreter::new();

        interpreter.execute_block(program.statements)
    }

    pub fn execute_block(&mut self, statements: Vec<Statement>) -> Result<(), LangError> {
        for statement in statements {
            self.execute_statement(statement)?;
        }

        Ok(())
    }

    pub fn execute_statement(&mut self, statement: Statement) -> Result<(), LangError> {
        match statement {
            Statement::Print { expression } => self.handle_print(&expression),

            Statement::StateDeclaration {
                identifier,
                expression,
            } => self.handle_state_declaration(identifier, expression),

            Statement::Conditional {
                condition,
                statements,
                else_statements,
            } => self.handle_conditional_statements(condition, statements, else_statements),

            Statement::ActionDeclaration {
                identifier,
                parameters,
                statements,
            } => self.handle_action_declaration(identifier, parameters, statements),

            Statement::Expression { expression } => {
                self.evaluate_expression(&expression)?;
                Ok(())
            }
        }
    }

    pub fn evaluate_expression(&mut self, expression: &Expression) -> Result<Value, LangError> {
        match expression {
            Expression::Literal(literal) => match literal {
                Literal::Number(number) => Ok(Value::Number(*number)),
                Literal::String(string) => Ok(Value::String(string.clone())),
                Literal::Boolean(boolean) => Ok(Value::Boolean(*boolean)),
                Literal::None => Ok(Value::None),
            },

            Expression::List(list) => {
                let mut list_values: Vec<Value> = Vec::new();

                for expression in list {
                    let new_value: Value = self.evaluate_expression(expression)?;

                    list_values.push(new_value);
                }

                Ok(Value::List(Rc::new(RefCell::new(list_values))))
            }

            Expression::Object(object) => {
                let mut fields: HashMap<String, Value> = HashMap::new();

                for (identifier, expression) in object {
                    let new_value: Value = self.evaluate_expression(expression)?;

                    fields.insert(identifier.clone(), new_value);
                }

                Ok(Value::Object(Rc::new(RefCell::new(fields))))
            }

            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left_value: Value = self.evaluate_expression(left)?;
                let right_value: Value = self.evaluate_expression(right)?;

                left_value.apply_binary_operator(operator, right_value)
            }

            Expression::Unary {
                operator,
                expression,
            } => {
                let value: Value = self.evaluate_expression(expression)?;

                value.apply_unary_operator(operator)
            }

            Expression::Identifier(identifier) => {
                let value: Option<Value> = self.environment.borrow().get(identifier);

                if let Some(value) = value {
                    return Ok(value.clone());
                } else {
                    return Err(
                        InterpreterError::NotFound(format!("'{identifier}' not found")).into(),
                    );
                }
            }

            Expression::Member {
                expression,
                property,
            } => {
                let value: Value = self.evaluate_expression(expression)?;
                let native_property: Option<Value> =
                    Interpreter::get_native_property(&value, property)?;

                if let Some(native_property) = native_property {
                    return Ok(native_property);
                }

                if let Some(method) = Interpreter::get_native_method(&value, property) {
                    return Ok(Value::BoundMethod {
                        receiver: Box::new(value),
                        method,
                    });
                }

                if let Value::Object(map) = value {
                    if let Some(value) = map.borrow().get(property) {
                        return Ok(value.clone());
                    }
                }

                Err(InterpreterError::NotFound(format!("Property '{property}' not found")).into())
            }

            Expression::Call { callee, arguments } => {
                let callee_value: Value = self.evaluate_expression(callee)?;
                let arguments: Vec<Value> = arguments
                    .iter()
                    .map(|arg| self.evaluate_expression(arg))
                    .collect::<Result<Vec<_>, _>>()?;

                match callee_value {
                    Value::BoundMethod { receiver, method } => method(*receiver, arguments),

                    Value::Action(action) => self.handle_action_execution(action, arguments),

                    _ => Err(InterpreterError::NotCallable(format!(
                        "'{callee_value}' is not callable"
                    ))
                    .into()),
                }
            }
            Expression::Assignment { target, expression } => {
                let value: Value = self.evaluate_expression(expression)?;

                match &**target {
                    Expression::Identifier(name) => {
                        self.environment
                            .borrow_mut()
                            .assign(name.clone(), value.clone())?;

                        Ok(value)
                    }

                    _ => Err(InterpreterError::InvalidAssignment(format!(
                        "'{target}' is not assignable"
                    ))
                    .into()),
                }
            }
        }
    }

    pub fn get_native_method(value: &Value, name: &str) -> Option<NativeMethod> {
        match value {
            Value::List(_) => get_list_methods().get(name).copied(),
            Value::String(_) => get_string_methods().get(name).copied(),
            _ => None,
        }
    }

    pub fn get_native_property(value: &Value, name: &str) -> Result<Option<Value>, LangError> {
        match value {
            Value::List(_) => {
                if let Some(function) = get_list_properties().get(name) {
                    return Ok(Some(function(value.clone())?));
                }
                Ok(None)
            }
            Value::String(_) => {
                if let Some(function) = get_string_properties().get(name) {
                    return Ok(Some(function(value.clone())?));
                }
                Ok(None)
            }
            _ => Ok(None),
        }
    }
}
