use crate::{
    abstract_syntax_tree::{expression::Expression, literal::Literal, statement::Statement},
    errors::{interpreter_error::InterpreterError, lang_error::LangError},
    interpreter::{environment::Environment, value::value::Value},
};

pub struct Interpreter {
    pub environment: Environment,
}

impl Interpreter {
    fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn execute(statements: Vec<Statement>) -> Result<(), LangError> {
        let mut interpreter: Interpreter = Interpreter::new();

        interpreter.execute_block(statements)
    }

    pub fn execute_block(&mut self, statements: Vec<Statement>) -> Result<(), LangError> {
        for statement in statements {
            self.execute_statement(statement)?;
        }
        Ok(())
    }

    fn execute_statement(&mut self, statement: Statement) -> Result<(), LangError> {
        match statement {
            Statement::Print { expression } => self.handle_print(&expression),

            Statement::VariableDeclaration {
                identifier,
                expression,
            } => self.handle_variable_declaration(identifier, expression),

            Statement::VariableAssignation {
                identifier,
                expression,
            } => self.handle_variable_assignation(identifier, expression),

            Statement::Conditional {
                condition,
                statements,
                else_statements,
            } => self.handle_conditional_statements(condition, statements, else_statements),
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

                Ok(Value::List(list_values))
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
                let value: Option<&Value> = self.environment.get_variable(identifier);

                if let Some(value) = value {
                    return Ok(value.clone());
                } else {
                    return Err(InterpreterError::NotFound(format!(
                        "Variable '{identifier}' not found"
                    ))
                    .into());
                }
            }
        }
    }
}
